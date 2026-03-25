use core::arch::x86_64::*;

use crate::base16::config::Base16EncodeConfig;

/// Encodes bytes into base16 (hex) characters using AVX-512.
///
/// Processes 32 input bytes → 64 output chars per iteration.
/// Any remaining bytes after the SIMD loop are left to the caller.
///
/// Returns the number of output bytes written to `dst`.
#[target_feature(enable = "avx512f,avx512bw")]
#[inline]
#[allow(unsafe_op_in_unsafe_fn)]
pub(crate) unsafe fn avx512_encode_into(
    config: &Base16EncodeConfig,
    dst: &mut [u8],
    src: &[u8],
) -> usize {
    // Broadcast the 16-char alphabet into all four 128-bit lanes.
    let alphabet =
        _mm512_broadcast_i32x4(_mm_loadu_si128(config.alphabet.as_ptr() as *const __m128i));
    let mask_0f = _mm512_set1_epi16(0x000F);

    // After packus_epi16(hi, lo) the per-lane layout is:
    //   [h0,h1,...,h7, l0,l1,...,l7]
    //
    // This shuffle interleaves to the correct output order:
    //   [h0,l0, h1,l1, ..., h7,l7]
    let interleave_shuf = _mm512_broadcast_i32x4(_mm_set_epi8(
        15, 7, 14, 6, 13, 5, 12, 4, 11, 3, 10, 2, 9, 1, 8, 0,
    ));

    let mut src_offset = 0usize;
    let mut dst_offset = 0usize;

    while src_offset + 32 <= src.len() {
        let input = _mm256_loadu_si256(src.as_ptr().add(src_offset) as *const __m256i);

        // Zero-extend 32 bytes to 32 × 16-bit words; isolates each byte so
        // srli_epi16 cannot leak bits from a neighbouring byte.
        let wide = _mm512_cvtepu8_epi16(input);

        let hi = _mm512_srli_epi16(wide, 4);
        let lo = _mm512_and_si512(wide, mask_0f);

        // Pack 16-bit lanes back to bytes (per 128-bit lane), then interleave hi/lo.
        let packed = _mm512_packus_epi16(hi, lo);
        let nibbles = _mm512_shuffle_epi8(packed, interleave_shuf);

        let chars = _mm512_shuffle_epi8(alphabet, nibbles);

        _mm512_storeu_si512(dst.as_mut_ptr().add(dst_offset) as *mut __m512i, chars);

        src_offset += 32;
        dst_offset += 64;
    }

    dst_offset
}
