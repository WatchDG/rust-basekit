use core::arch::x86_64::*;

use crate::base16::config::Base16DecodeConfig;

/// Decodes base16 (hex) characters into bytes using SSSE3.
///
/// Processes 16 input hex chars → 8 output bytes per iteration.
/// On encountering invalid characters the function stops and returns the
/// number of output bytes written so far; the caller's scalar loop handles
/// precise error reporting for the remaining input.
///
/// Returns the number of output bytes written to `dst`.
#[target_feature(enable = "ssse3")]
#[inline]
#[allow(unsafe_op_in_unsafe_fn)]
pub(crate) unsafe fn ssse3_decode_into(
    config: &Base16DecodeConfig,
    dst: &mut [u8],
    src: &[u8],
) -> usize {
    // Load 8 × 16-byte slices of the 128-entry decode table.
    let table_ptr = config.decode_table.as_ptr() as *const __m128i;
    let tbl0 = _mm_loadu_si128(table_ptr);
    let tbl1 = _mm_loadu_si128(table_ptr.add(1));
    let tbl2 = _mm_loadu_si128(table_ptr.add(2));
    let tbl3 = _mm_loadu_si128(table_ptr.add(3));
    let tbl4 = _mm_loadu_si128(table_ptr.add(4));
    let tbl5 = _mm_loadu_si128(table_ptr.add(5));
    let tbl6 = _mm_loadu_si128(table_ptr.add(6));
    let tbl7 = _mm_loadu_si128(table_ptr.add(7));

    // maddubs weights: [16, 1] repeating — computes hi_nibble*16 + lo_nibble.
    let maddubs_weights = _mm_set1_epi16(0x0110);

    // Shuffle to extract the low byte of each 16-bit lane → 8 packed output bytes.
    let pack_shuf = _mm_set_epi8(-1, -1, -1, -1, -1, -1, -1, -1, 14, 12, 10, 8, 6, 4, 2, 0);

    let invalid = _mm_set1_epi8(-1);

    let mut src_offset = 0usize;
    let mut dst_offset = 0usize;

    while src_offset + 16 <= src.len() {
        let input = _mm_loadu_si128(src.as_ptr().add(src_offset) as *const __m128i);

        // Any byte >= 128 is outside the decode table; bail to scalar.
        if _mm_movemask_epi8(input) != 0 {
            break;
        }

        // 128-entry table lookup via 8 × pshufb, selecting by upper nibble bits [6:4].
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

        // Any lane with 0xFF means an invalid character; bail to scalar.
        if _mm_movemask_epi8(_mm_cmpeq_epi8(decoded, invalid)) != 0 {
            break;
        }

        // Pack pairs of nibbles: hi_nibble * 16 + lo_nibble → 8 × 16-bit values.
        let combined = _mm_maddubs_epi16(decoded, maddubs_weights);

        // Extract the low byte of each 16-bit lane into 8 contiguous bytes.
        let packed = _mm_shuffle_epi8(combined, pack_shuf);

        _mm_storel_epi64(dst.as_mut_ptr().add(dst_offset) as *mut __m128i, packed);

        src_offset += 16;
        dst_offset += 8;
    }

    dst_offset
}
