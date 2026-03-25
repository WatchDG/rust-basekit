use core::arch::x86_64::*;

use super::super::decode_full_group_into::decode_full_group_into;
use crate::base32::config::Base32DecodeConfig;
use crate::base32::error::Base32Error;

/// Decodes full 4-group blocks (src.len() is a multiple of 32, no padding).
/// Each 32 base32 characters → 20 output bytes (4 groups of 8, 2 per 128-bit lane).
/// Tail groups and padding are handled by the caller.
#[target_feature(enable = "avx2")]
#[inline]
#[allow(unsafe_op_in_unsafe_fn)]
pub(crate) unsafe fn avx2_decode_full_groups_into(
    config: &Base32DecodeConfig,
    dst: &mut [u8],
    src: &[u8],
) -> Result<usize, Base32Error> {
    debug_assert_eq!(src.len() % 32, 0);

    // Broadcast each 16-byte table block into both 128-bit lanes of a 256-bit register.
    // _mm256_shuffle_epi8 operates per-lane, so both lanes need the same 16-byte sub-table.
    let table_ptr = config.decode_table.as_ptr() as *const __m128i;
    let tbl0 = _mm256_broadcastsi128_si256(_mm_loadu_si128(table_ptr));
    let tbl1 = _mm256_broadcastsi128_si256(_mm_loadu_si128(table_ptr.add(1)));
    let tbl2 = _mm256_broadcastsi128_si256(_mm_loadu_si128(table_ptr.add(2)));
    let tbl3 = _mm256_broadcastsi128_si256(_mm_loadu_si128(table_ptr.add(3)));
    let tbl4 = _mm256_broadcastsi128_si256(_mm_loadu_si128(table_ptr.add(4)));
    let tbl5 = _mm256_broadcastsi128_si256(_mm_loadu_si128(table_ptr.add(5)));
    let tbl6 = _mm256_broadcastsi128_si256(_mm_loadu_si128(table_ptr.add(6)));
    let tbl7 = _mm256_broadcastsi128_si256(_mm_loadu_si128(table_ptr.add(7)));

    // Same per-lane shuffle as SSSE3 base32: compacts 16 decoded values into 10 output bytes per lane.
    //
    // After bit-packing each i64 lane holds 5 output bytes in LE order:
    //   [b4, b3, b2, b1, b0, X, X, X]
    //
    // This shuffle reverses each 5-byte group and places group 1 right after group 0:
    //   [b0_g0, b1_g0, b2_g0, b3_g0, b4_g0, b0_g1, b1_g1, b2_g1, b3_g1, b4_g1, 0, 0, 0, 0, 0, 0]
    let pack_shuf_128 = _mm_set_epi8(-1, -1, -1, -1, -1, -1, 8, 9, 10, 11, 12, 0, 1, 2, 3, 4);
    let pack_shuf = _mm256_broadcastsi128_si256(pack_shuf_128);

    let mut src_offset = 0usize;
    let mut dst_offset = 0usize;

    while src_offset + 32 <= src.len() {
        let input = _mm256_loadu_si256(src.as_ptr().add(src_offset) as *const __m256i);

        // Any byte >= 128 → invalid ASCII, fall back to scalar for this 32-byte block.
        if _mm256_movemask_epi8(input) != 0 {
            let mut written = 0usize;
            for group_offset in (0..32usize).step_by(8) {
                written += decode_full_group_into(
                    config,
                    &src[src_offset + group_offset..src_offset + group_offset + 8],
                    src_offset + group_offset,
                    dst,
                    dst_offset + written,
                )?;
            }
            dst_offset += written;
            src_offset += 32;
            continue;
        }

        // Table lookup via 8 × vpshufb.
        //
        // upper = bits 6:4 of each input byte (selects one of 8 decode-table blocks).
        // low   = bits 3:0 of each input byte (index within the 16-byte block).
        // sel_k = 0xFF where input's upper nibble == k, else 0x00.
        // r_k   = vpshufb(tbl_k, low) & sel_k
        // decoded = r_0 | r_1 | ... | r_7
        let upper = _mm256_and_si256(input, _mm256_set1_epi8(0x70u8 as i8));
        let low = _mm256_and_si256(input, _mm256_set1_epi8(0x0F));

        let sel0 = _mm256_cmpeq_epi8(upper, _mm256_setzero_si256());
        let sel1 = _mm256_cmpeq_epi8(upper, _mm256_set1_epi8(0x10u8 as i8));
        let sel2 = _mm256_cmpeq_epi8(upper, _mm256_set1_epi8(0x20u8 as i8));
        let sel3 = _mm256_cmpeq_epi8(upper, _mm256_set1_epi8(0x30u8 as i8));
        let sel4 = _mm256_cmpeq_epi8(upper, _mm256_set1_epi8(0x40u8 as i8));
        let sel5 = _mm256_cmpeq_epi8(upper, _mm256_set1_epi8(0x50u8 as i8));
        let sel6 = _mm256_cmpeq_epi8(upper, _mm256_set1_epi8(0x60u8 as i8));
        let sel7 = _mm256_cmpeq_epi8(upper, _mm256_set1_epi8(0x70u8 as i8));

        let r0 = _mm256_and_si256(_mm256_shuffle_epi8(tbl0, low), sel0);
        let r1 = _mm256_and_si256(_mm256_shuffle_epi8(tbl1, low), sel1);
        let r2 = _mm256_and_si256(_mm256_shuffle_epi8(tbl2, low), sel2);
        let r3 = _mm256_and_si256(_mm256_shuffle_epi8(tbl3, low), sel3);
        let r4 = _mm256_and_si256(_mm256_shuffle_epi8(tbl4, low), sel4);
        let r5 = _mm256_and_si256(_mm256_shuffle_epi8(tbl5, low), sel5);
        let r6 = _mm256_and_si256(_mm256_shuffle_epi8(tbl6, low), sel6);
        let r7 = _mm256_and_si256(_mm256_shuffle_epi8(tbl7, low), sel7);

        let decoded = _mm256_or_si256(
            _mm256_or_si256(_mm256_or_si256(r0, r1), _mm256_or_si256(r2, r3)),
            _mm256_or_si256(_mm256_or_si256(r4, r5), _mm256_or_si256(r6, r7)),
        );

        // Any 0xFF in decoded means an invalid character in the table.
        if _mm256_movemask_epi8(_mm256_cmpeq_epi8(decoded, _mm256_set1_epi8(-1))) != 0 {
            let mut written = 0usize;
            for group_offset in (0..32usize).step_by(8) {
                written += decode_full_group_into(
                    config,
                    &src[src_offset + group_offset..src_offset + group_offset + 8],
                    src_offset + group_offset,
                    dst,
                    dst_offset + written,
                )?;
            }
            dst_offset += written;
            src_offset += 32;
            continue;
        }

        // Pack 32 × 5-bit decoded values into 20 output bytes (Muła/Lemire algorithm, per lane).
        //
        // Step A — maddubs: merge adjacent pairs (a, b) → a*32 + b (10-bit in 16-bit field).
        //   decoded[i] ∈ [0..31], so a*32+b ≤ 1023, no i16 saturation.
        let t0 = _mm256_maddubs_epi16(decoded, _mm256_set1_epi16(0x0120));

        // Step B — madd: merge adjacent 10-bit pairs (c, d) → c*1024 + d (20-bit in 32-bit field).
        //   Each i32 lane now holds 4 decoded values packed into 20 bits.
        let t1 = _mm256_madd_epi16(t0, _mm256_set1_epi32(0x00010400u32 as i32));

        // Step C — combine two 20-bit halves into a 40-bit (5-byte) value per i64 lane.
        //
        // After madd, each i64 lane is [q0 (i32), q1 (i32)] (LE).
        //   slli_epi64(t1, 20) → bits [39:20] = q0
        //   srli_epi64(t1, 32) → bits [19:0]  = q1
        //   OR → bits [39:0] = q0 << 20 | q1 (5 output bytes, in LE byte order)
        let combined = _mm256_or_si256(_mm256_slli_epi64(t1, 20), _mm256_srli_epi64(t1, 32));

        // Step D — pshufb: reverse LE bytes within each 5-byte group, compact two groups per lane.
        let packed = _mm256_shuffle_epi8(combined, pack_shuf);

        // Write 20 bytes: 10 from the low 128-bit lane, then 10 from the high 128-bit lane.
        let out_ptr = dst.as_mut_ptr().add(dst_offset);

        let lo = _mm256_castsi256_si128(packed);
        _mm_storel_epi64(out_ptr as *mut __m128i, lo);
        let lo_shifted = _mm_srli_si128(lo, 8);
        let lo_word = _mm_cvtsi128_si32(lo_shifted) as u16;
        core::ptr::write_unaligned(out_ptr.add(8) as *mut u16, lo_word);

        let hi = _mm256_extracti128_si256(packed, 1);
        _mm_storel_epi64(out_ptr.add(10) as *mut __m128i, hi);
        let hi_shifted = _mm_srli_si128(hi, 8);
        let hi_word = _mm_cvtsi128_si32(hi_shifted) as u16;
        core::ptr::write_unaligned(out_ptr.add(18) as *mut u16, hi_word);

        src_offset += 32;
        dst_offset += 20;
    }

    Ok(dst_offset)
}
