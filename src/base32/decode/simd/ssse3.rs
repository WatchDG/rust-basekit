use core::arch::x86_64::*;

use super::super::decode_full_group_into::decode_full_group_into;
use crate::base32::config::Base32DecodeConfig;
use crate::base32::error::Base32Error;

/// Decodes full 8-group blocks (src.len() is a multiple of 16, no padding).
/// Each 16 base32 characters → 10 output bytes (2 groups of 8).
/// Tail groups and padding are handled by the caller.
#[target_feature(enable = "ssse3")]
#[inline]
#[allow(unsafe_op_in_unsafe_fn)]
pub(crate) unsafe fn ssse3_decode_full_groups_into(
    config: &Base32DecodeConfig,
    dst: &mut [u8],
    src: &[u8],
) -> Result<usize, Base32Error> {
    debug_assert_eq!(src.len() % 16, 0);

    let table_ptr = config.decode_table.as_ptr() as *const __m128i;
    let tbl0 = _mm_loadu_si128(table_ptr);
    let tbl1 = _mm_loadu_si128(table_ptr.add(1));
    let tbl2 = _mm_loadu_si128(table_ptr.add(2));
    let tbl3 = _mm_loadu_si128(table_ptr.add(3));
    let tbl4 = _mm_loadu_si128(table_ptr.add(4));
    let tbl5 = _mm_loadu_si128(table_ptr.add(5));
    let tbl6 = _mm_loadu_si128(table_ptr.add(6));
    let tbl7 = _mm_loadu_si128(table_ptr.add(7));

    // pshufb shuffle: reverse 5-byte groups within each i64 lane and compact.
    //
    // After bit-packing, each i64 lane holds 5 output bytes in LE order:
    //   [b4, b3, b2, b1, b0, X, X, X]
    //
    // This shuffle reverses each group and places group 1 right after group 0:
    //   [b0_g0, b1_g0, b2_g0, b3_g0, b4_g0, b0_g1, b1_g1, b2_g1, b3_g1, b4_g1, 0, 0, 0, 0, 0, 0]
    let pack_shuf = _mm_set_epi8(-1, -1, -1, -1, -1, -1, 8, 9, 10, 11, 12, 0, 1, 2, 3, 4);

    let mut src_offset = 0usize;
    let mut dst_offset = 0usize;

    while src_offset + 16 <= src.len() {
        let input = _mm_loadu_si128(src.as_ptr().add(src_offset) as *const __m128i);

        if _mm_movemask_epi8(input) != 0 {
            let mut written = 0usize;
            for group_offset in (0..16).step_by(8) {
                written += decode_full_group_into(
                    config,
                    &src[src_offset + group_offset..src_offset + group_offset + 8],
                    src_offset + group_offset,
                    dst,
                    dst_offset + written,
                )?;
            }
            dst_offset += written;
            src_offset += 16;
            continue;
        }

        // Table lookup via 8 × pshufb (identical to base64 SSSE3 decode).
        let upper = _mm_and_si128(input, _mm_set1_epi8(0x70u8 as i8));
        let low = _mm_and_si128(input, _mm_set1_epi8(0x0F));

        let sel0 = _mm_cmpeq_epi8(upper, _mm_setzero_si128());
        let sel1 = _mm_cmpeq_epi8(upper, _mm_set1_epi8(0x10u8 as i8));
        let sel2 = _mm_cmpeq_epi8(upper, _mm_set1_epi8(0x20u8 as i8));
        let sel3 = _mm_cmpeq_epi8(upper, _mm_set1_epi8(0x30u8 as i8));
        let sel4 = _mm_cmpeq_epi8(upper, _mm_set1_epi8(0x40u8 as i8));
        let sel5 = _mm_cmpeq_epi8(upper, _mm_set1_epi8(0x50u8 as i8));
        let sel6 = _mm_cmpeq_epi8(upper, _mm_set1_epi8(0x60u8 as i8));
        let sel7 = _mm_cmpeq_epi8(upper, _mm_set1_epi8(0x70u8 as i8));

        let r0 = _mm_and_si128(_mm_shuffle_epi8(tbl0, low), sel0);
        let r1 = _mm_and_si128(_mm_shuffle_epi8(tbl1, low), sel1);
        let r2 = _mm_and_si128(_mm_shuffle_epi8(tbl2, low), sel2);
        let r3 = _mm_and_si128(_mm_shuffle_epi8(tbl3, low), sel3);
        let r4 = _mm_and_si128(_mm_shuffle_epi8(tbl4, low), sel4);
        let r5 = _mm_and_si128(_mm_shuffle_epi8(tbl5, low), sel5);
        let r6 = _mm_and_si128(_mm_shuffle_epi8(tbl6, low), sel6);
        let r7 = _mm_and_si128(_mm_shuffle_epi8(tbl7, low), sel7);

        let decoded = _mm_or_si128(
            _mm_or_si128(_mm_or_si128(r0, r1), _mm_or_si128(r2, r3)),
            _mm_or_si128(_mm_or_si128(r4, r5), _mm_or_si128(r6, r7)),
        );

        if _mm_movemask_epi8(_mm_cmpeq_epi8(decoded, _mm_set1_epi8(-1))) != 0 {
            let mut written = 0usize;
            for group_offset in (0..16).step_by(8) {
                written += decode_full_group_into(
                    config,
                    &src[src_offset + group_offset..src_offset + group_offset + 8],
                    src_offset + group_offset,
                    dst,
                    dst_offset + written,
                )?;
            }
            dst_offset += written;
            src_offset += 16;
            continue;
        }

        // Pack 16 × 5-bit decoded values into 10 output bytes.
        //
        // Step A — maddubs: merge adjacent pairs (a, b) → a*32 + b (10-bit in i16).
        //   decoded[i] ∈ [0..31], so a*32+b ≤ 1023, no i16 saturation.
        let t0 = _mm_maddubs_epi16(decoded, _mm_set1_epi16(0x0120));

        // Step B — madd: merge adjacent 10-bit pairs (c, d) → c*1024 + d (20-bit in i32).
        //   Each i32 lane now holds 4 decoded values packed into 20 bits.
        let t1 = _mm_madd_epi16(t0, _mm_set1_epi32(0x00010400u32 as i32));

        // Step C — combine two 20-bit halves into a 40-bit (5-byte) value per i64 lane.
        //
        // After madd, each i64 lane is [q0 (i32), q1 (i32)] (LE).
        //   _mm_slli_epi64(t1, 20) → bits [39:20] = q0
        //   _mm_srli_epi64(t1, 32) → bits [19:0]  = q1
        //   OR → bits [39:0] = q0 << 20 | q1 (5 output bytes, in LE byte order)
        let combined = _mm_or_si128(_mm_slli_epi64(t1, 20), _mm_srli_epi64(t1, 32));

        // Step D — pshufb: reverse LE bytes within each 5-byte group, compact two groups.
        let packed = _mm_shuffle_epi8(combined, pack_shuf);

        let out_ptr = dst.as_mut_ptr().add(dst_offset);
        _mm_storel_epi64(out_ptr as *mut __m128i, packed);
        let shifted = _mm_srli_si128(packed, 8);
        let word = _mm_cvtsi128_si32(shifted) as u16;
        core::ptr::write_unaligned(out_ptr.add(8) as *mut u16, word);

        src_offset += 16;
        dst_offset += 10;
    }

    Ok(dst_offset)
}
