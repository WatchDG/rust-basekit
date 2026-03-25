use core::arch::x86_64::*;

use crate::base16::config::Base16DecodeConfig;

/// Decodes base16 (hex) characters into bytes using AVX2.
///
/// Processes 32 input hex chars → 16 output bytes per iteration.
/// On encountering invalid characters the function stops and returns the
/// number of output bytes written so far; the caller's scalar loop handles
/// precise error reporting for the remaining input.
///
/// Returns the number of output bytes written to `dst`.
#[target_feature(enable = "avx2")]
#[inline]
#[allow(unsafe_op_in_unsafe_fn)]
pub(crate) unsafe fn avx2_decode_into(
    config: &Base16DecodeConfig,
    dst: &mut [u8],
    src: &[u8],
) -> usize {
    // Broadcast each 16-byte slice of the 128-entry decode table into both lanes.
    let table_ptr = config.decode_table.as_ptr() as *const __m128i;
    let tbl0 = _mm256_broadcastsi128_si256(_mm_loadu_si128(table_ptr));
    let tbl1 = _mm256_broadcastsi128_si256(_mm_loadu_si128(table_ptr.add(1)));
    let tbl2 = _mm256_broadcastsi128_si256(_mm_loadu_si128(table_ptr.add(2)));
    let tbl3 = _mm256_broadcastsi128_si256(_mm_loadu_si128(table_ptr.add(3)));
    let tbl4 = _mm256_broadcastsi128_si256(_mm_loadu_si128(table_ptr.add(4)));
    let tbl5 = _mm256_broadcastsi128_si256(_mm_loadu_si128(table_ptr.add(5)));
    let tbl6 = _mm256_broadcastsi128_si256(_mm_loadu_si128(table_ptr.add(6)));
    let tbl7 = _mm256_broadcastsi128_si256(_mm_loadu_si128(table_ptr.add(7)));

    let maddubs_weights = _mm256_set1_epi16(0x0110);

    // Per-lane: extract the low byte of each 16-bit lane → 8 packed bytes in qword 0.
    let pack_shuf = _mm256_broadcastsi128_si256(_mm_set_epi8(
        -1, -1, -1, -1, -1, -1, -1, -1, 14, 12, 10, 8, 6, 4, 2, 0,
    ));

    let invalid = _mm256_set1_epi8(-1);

    let mut src_offset = 0usize;
    let mut dst_offset = 0usize;

    while src_offset + 32 <= src.len() {
        let input = _mm256_loadu_si256(src.as_ptr().add(src_offset) as *const __m256i);

        if _mm256_movemask_epi8(input) != 0 {
            break;
        }

        // 128-entry table lookup via 8 × vpshufb.
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

        if _mm256_movemask_epi8(_mm256_cmpeq_epi8(decoded, invalid)) != 0 {
            break;
        }

        // Pack pairs of nibbles: hi_nibble * 16 + lo_nibble → 16 × 16-bit values.
        let combined = _mm256_maddubs_epi16(decoded, maddubs_weights);

        // Extract the low byte of each 16-bit lane → 8 bytes per lane (in qword 0).
        let packed = _mm256_shuffle_epi8(combined, pack_shuf);

        // Gather qword 0 from each lane into contiguous 16 bytes.
        // qword 0 (lane 0) → position 0, qword 2 (lane 1) → position 1.
        let result = _mm256_permute4x64_epi64(packed, 0b00_00_10_00);

        _mm_storeu_si128(
            dst.as_mut_ptr().add(dst_offset) as *mut __m128i,
            _mm256_castsi256_si128(result),
        );

        src_offset += 32;
        dst_offset += 16;
    }

    dst_offset
}
