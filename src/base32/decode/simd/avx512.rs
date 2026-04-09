use core::arch::x86_64::*;

use super::super::decode_full_group_into::decode_full_group_into;
use crate::base32::config::Base32DecodeConfig;
use crate::base32::error::Base32Error;

/// Decodes full 8-group blocks (src.len() is a multiple of 64, no padding).
/// Each 64 base32 characters → 40 output bytes (8 groups of 8, 2 per 128-bit lane × 4 lanes).
/// Tail groups and padding are handled by the caller.
#[target_feature(enable = "avx512f,avx512bw")]
#[inline]
#[allow(unsafe_op_in_unsafe_fn)]
pub(crate) unsafe fn avx512_decode_full_groups_into(
    config: &Base32DecodeConfig,
    dst: &mut [u8],
    src: &[u8],
) -> Result<usize, Base32Error> {
    debug_assert_eq!(src.len() % 64, 0);

    // Broadcast each 16-byte table block into all four 128-bit lanes of a 512-bit register.
    // _mm512_shuffle_epi8 operates per 16-byte lane, so all lanes need the same sub-table.
    let table_ptr = config.decode_table.as_ptr() as *const __m128i;
    let tbl0 = _mm512_broadcast_i32x4(_mm_loadu_si128(table_ptr));
    let tbl1 = _mm512_broadcast_i32x4(_mm_loadu_si128(table_ptr.add(1)));
    let tbl2 = _mm512_broadcast_i32x4(_mm_loadu_si128(table_ptr.add(2)));
    let tbl3 = _mm512_broadcast_i32x4(_mm_loadu_si128(table_ptr.add(3)));
    let tbl4 = _mm512_broadcast_i32x4(_mm_loadu_si128(table_ptr.add(4)));
    let tbl5 = _mm512_broadcast_i32x4(_mm_loadu_si128(table_ptr.add(5)));
    let tbl6 = _mm512_broadcast_i32x4(_mm_loadu_si128(table_ptr.add(6)));
    let tbl7 = _mm512_broadcast_i32x4(_mm_loadu_si128(table_ptr.add(7)));

    // Same per-lane shuffle as SSSE3/AVX2 base32: compacts 16 decoded values into 10 output
    // bytes per lane (2 groups × 5 bytes each).
    //
    // After bit-packing each i64 lane holds 5 output bytes in LE order:
    //   [b4, b3, b2, b1, b0, X, X, X]
    //
    // This shuffle reverses each 5-byte group and places group 1 right after group 0:
    //   [b0_g0, b1_g0, b2_g0, b3_g0, b4_g0, b0_g1, b1_g1, b2_g1, b3_g1, b4_g1, 0, 0, 0, 0, 0, 0]
    let pack_shuf_128 = _mm_set_epi8(-1, -1, -1, -1, -1, -1, 8, 9, 10, 11, 12, 0, 1, 2, 3, 4);
    let pack_shuf = _mm512_broadcast_i32x4(pack_shuf_128);

    let mut src_offset = 0usize;
    let mut dst_offset = 0usize;

    while src_offset + 64 <= src.len() {
        let input = _mm512_loadu_si512(src.as_ptr().add(src_offset) as *const __m512i);

        // Any byte >= 128 → invalid ASCII; fall back to scalar for this 64-byte block.
        if _mm512_movepi8_mask(input) != 0 {
            let mut written = 0usize;
            for group_offset in (0..64usize).step_by(8) {
                written += decode_full_group_into(
                    config,
                    dst,
                    dst_offset + written,
                    &src[src_offset + group_offset..src_offset + group_offset + 8],
                    src_offset + group_offset,
                )?;
            }
            dst_offset += written;
            src_offset += 64;
            continue;
        }

        // Table lookup via 8 × vpshufb (AVX-512BW, per-lane).
        //
        // upper = bits 6:4 of each input byte (selects one of 8 decode-table blocks).
        // low   = bits 3:0 of each input byte (index within the 16-byte block).
        // mask_k = bitmask where input's upper nibble == k.
        // r_k = _mm512_maskz_shuffle_epi8(mask_k, tbl_k, low)
        //       ≡ pshufb(tbl_k, low) with bytes zeroed where mask_k bit is 0.
        //       This replaces the two-instruction (pshufb + and) sequence used in AVX2.
        // decoded = r_0 | r_1 | ... | r_7
        let upper = _mm512_and_si512(input, _mm512_set1_epi8(0x70u8 as i8));
        let low = _mm512_and_si512(input, _mm512_set1_epi8(0x0F));

        let mask0 = _mm512_cmpeq_epi8_mask(upper, _mm512_setzero_si512());
        let mask1 = _mm512_cmpeq_epi8_mask(upper, _mm512_set1_epi8(0x10u8 as i8));
        let mask2 = _mm512_cmpeq_epi8_mask(upper, _mm512_set1_epi8(0x20u8 as i8));
        let mask3 = _mm512_cmpeq_epi8_mask(upper, _mm512_set1_epi8(0x30u8 as i8));
        let mask4 = _mm512_cmpeq_epi8_mask(upper, _mm512_set1_epi8(0x40u8 as i8));
        let mask5 = _mm512_cmpeq_epi8_mask(upper, _mm512_set1_epi8(0x50u8 as i8));
        let mask6 = _mm512_cmpeq_epi8_mask(upper, _mm512_set1_epi8(0x60u8 as i8));
        let mask7 = _mm512_cmpeq_epi8_mask(upper, _mm512_set1_epi8(0x70u8 as i8));

        let r0 = _mm512_maskz_shuffle_epi8(mask0, tbl0, low);
        let r1 = _mm512_maskz_shuffle_epi8(mask1, tbl1, low);
        let r2 = _mm512_maskz_shuffle_epi8(mask2, tbl2, low);
        let r3 = _mm512_maskz_shuffle_epi8(mask3, tbl3, low);
        let r4 = _mm512_maskz_shuffle_epi8(mask4, tbl4, low);
        let r5 = _mm512_maskz_shuffle_epi8(mask5, tbl5, low);
        let r6 = _mm512_maskz_shuffle_epi8(mask6, tbl6, low);
        let r7 = _mm512_maskz_shuffle_epi8(mask7, tbl7, low);

        let decoded = _mm512_or_si512(
            _mm512_or_si512(_mm512_or_si512(r0, r1), _mm512_or_si512(r2, r3)),
            _mm512_or_si512(_mm512_or_si512(r4, r5), _mm512_or_si512(r6, r7)),
        );

        // Any 0xFF in decoded means an invalid character in the table.
        if _mm512_cmpeq_epi8_mask(decoded, _mm512_set1_epi8(-1)) != 0 {
            let mut written = 0usize;
            for group_offset in (0..64usize).step_by(8) {
                written += decode_full_group_into(
                    config,
                    dst,
                    dst_offset + written,
                    &src[src_offset + group_offset..src_offset + group_offset + 8],
                    src_offset + group_offset,
                )?;
            }
            dst_offset += written;
            src_offset += 64;
            continue;
        }

        // Pack 64 × 5-bit decoded values into 40 output bytes (Muła/Lemire algorithm, per lane).
        //
        // Step A — maddubs: merge adjacent pairs (a, b) → a*32 + b (10-bit in 16-bit field).
        //   decoded[i] ∈ [0..31], so a*32+b ≤ 1023, no i16 saturation.
        let t0 = _mm512_maddubs_epi16(decoded, _mm512_set1_epi16(0x0120));

        // Step B — madd: merge adjacent 10-bit pairs (c, d) → c*1024 + d (20-bit in 32-bit field).
        //   Each i32 lane now holds 4 decoded values packed into 20 bits.
        let t1 = _mm512_madd_epi16(t0, _mm512_set1_epi32(0x00010400u32 as i32));

        // Step C — combine two 20-bit halves into a 40-bit (5-byte) value per i64 lane.
        //
        // After madd, each i64 lane is [q0 (i32), q1 (i32)] (LE).
        //   slli_epi64(t1, 20) → bits [39:20] = q0
        //   srli_epi64(t1, 32) → bits [19:0]  = q1
        //   OR → bits [39:0] = q0 << 20 | q1 (5 output bytes, in LE byte order)
        let combined = _mm512_or_si512(_mm512_slli_epi64(t1, 20), _mm512_srli_epi64(t1, 32));

        // Step D — pshufb: reverse LE bytes within each 5-byte group, compact two groups per lane.
        let packed = _mm512_shuffle_epi8(combined, pack_shuf);

        // Write 40 bytes: 10 bytes from each of the 4 × 128-bit lanes.
        let out_ptr = dst.as_mut_ptr().add(dst_offset);

        // Lane 0 (bytes 0..9)
        let lane0 = _mm512_castsi512_si128(packed);
        _mm_storel_epi64(out_ptr as *mut __m128i, lane0);
        let lane0_hi = _mm_srli_si128(lane0, 8);
        core::ptr::write_unaligned(
            out_ptr.add(8) as *mut u16,
            _mm_cvtsi128_si32(lane0_hi) as u16,
        );

        // Lane 1 (bytes 10..19)
        let lane1 = _mm512_extracti32x4_epi32(packed, 1);
        _mm_storel_epi64(out_ptr.add(10) as *mut __m128i, lane1);
        let lane1_hi = _mm_srli_si128(lane1, 8);
        core::ptr::write_unaligned(
            out_ptr.add(18) as *mut u16,
            _mm_cvtsi128_si32(lane1_hi) as u16,
        );

        // Lane 2 (bytes 20..29)
        let lane2 = _mm512_extracti32x4_epi32(packed, 2);
        _mm_storel_epi64(out_ptr.add(20) as *mut __m128i, lane2);
        let lane2_hi = _mm_srli_si128(lane2, 8);
        core::ptr::write_unaligned(
            out_ptr.add(28) as *mut u16,
            _mm_cvtsi128_si32(lane2_hi) as u16,
        );

        // Lane 3 (bytes 30..39)
        let lane3 = _mm512_extracti32x4_epi32(packed, 3);
        _mm_storel_epi64(out_ptr.add(30) as *mut __m128i, lane3);
        let lane3_hi = _mm_srli_si128(lane3, 8);
        core::ptr::write_unaligned(
            out_ptr.add(38) as *mut u16,
            _mm_cvtsi128_si32(lane3_hi) as u16,
        );

        src_offset += 64;
        dst_offset += 40;
    }

    Ok(dst_offset)
}
