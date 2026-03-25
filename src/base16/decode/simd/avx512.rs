use core::arch::x86_64::*;

use crate::base16::config::Base16DecodeConfig;

/// Decodes base16 (hex) characters into bytes using AVX-512.
///
/// Processes 64 input hex chars -> 32 output bytes per iteration.
/// On encountering invalid characters the function stops and returns the
/// number of output bytes written so far; the caller's scalar loop handles
/// precise error reporting for the remaining input.
///
/// Returns the number of output bytes written to `dst`.
#[target_feature(enable = "avx512f,avx512bw")]
#[inline]
#[allow(unsafe_op_in_unsafe_fn)]
pub(crate) unsafe fn avx512_decode_into(
    config: &Base16DecodeConfig,
    dst: &mut [u8],
    src: &[u8],
) -> usize {
    // Broadcast each 16-byte slice of the 128-entry decode table into all four lanes.
    let table_ptr = config.decode_table.as_ptr() as *const __m128i;
    let tbl0 = _mm512_broadcast_i32x4(_mm_loadu_si128(table_ptr));
    let tbl1 = _mm512_broadcast_i32x4(_mm_loadu_si128(table_ptr.add(1)));
    let tbl2 = _mm512_broadcast_i32x4(_mm_loadu_si128(table_ptr.add(2)));
    let tbl3 = _mm512_broadcast_i32x4(_mm_loadu_si128(table_ptr.add(3)));
    let tbl4 = _mm512_broadcast_i32x4(_mm_loadu_si128(table_ptr.add(4)));
    let tbl5 = _mm512_broadcast_i32x4(_mm_loadu_si128(table_ptr.add(5)));
    let tbl6 = _mm512_broadcast_i32x4(_mm_loadu_si128(table_ptr.add(6)));
    let tbl7 = _mm512_broadcast_i32x4(_mm_loadu_si128(table_ptr.add(7)));

    let maddubs_weights = _mm512_set1_epi16(0x0110);

    // Per-lane: extract the low byte of each 16-bit lane -> 8 packed bytes in qword 0.
    let pack_shuf = _mm512_broadcast_i32x4(_mm_set_epi8(
        -1, -1, -1, -1, -1, -1, -1, -1, 14, 12, 10, 8, 6, 4, 2, 0,
    ));

    // Gather qwords 0, 2, 4, 6 (the packed results from each 128-bit lane) into
    // contiguous positions in the lower 256 bits.
    let perm_idx = _mm512_set_epi64(0, 0, 0, 0, 6, 4, 2, 0);

    let mut src_offset = 0usize;
    let mut dst_offset = 0usize;

    while src_offset + 64 <= src.len() {
        let input = _mm512_loadu_si512(src.as_ptr().add(src_offset) as *const __m512i);

        if _mm512_movepi8_mask(input) != 0 {
            break;
        }

        // 128-entry table lookup via 8 x masked vpshufb (AVX-512BW).
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

        if _mm512_cmpeq_epi8_mask(decoded, _mm512_set1_epi8(-1)) != 0 {
            break;
        }

        // Pack pairs of nibbles: hi_nibble * 16 + lo_nibble -> 32 x 16-bit values.
        let combined = _mm512_maddubs_epi16(decoded, maddubs_weights);

        // Extract the low byte of each 16-bit lane -> 8 bytes per lane (in qword 0).
        let packed = _mm512_shuffle_epi8(combined, pack_shuf);

        // Collect qwords 0,2,4,6 into contiguous 32 bytes in the lower half.
        let result = _mm512_permutexvar_epi64(perm_idx, packed);

        _mm256_storeu_si256(
            dst.as_mut_ptr().add(dst_offset) as *mut __m256i,
            _mm512_castsi512_si256(result),
        );

        src_offset += 64;
        dst_offset += 32;
    }

    dst_offset
}
