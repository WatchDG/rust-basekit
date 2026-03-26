use core::arch::x86_64::*;

use crate::base16::config::Base16EncodeConfig;
use crate::base16::error::Base16Error;

/// Encodes bytes into base16 (hex) characters using AVX2.
///
/// Processes 16 input bytes → 32 output chars per iteration.
/// Any remaining bytes after the SIMD loop are left to the caller.
///
/// Returns the number of output bytes written to `dst`.
#[target_feature(enable = "avx2")]
#[inline]
#[allow(unsafe_op_in_unsafe_fn)]
pub(crate) unsafe fn avx2_encode_into(
    config: &Base16EncodeConfig,
    dst: &mut [u8],
    src: &[u8],
) -> Result<usize, Base16Error> {
    // Broadcast the 16-char alphabet into both 128-bit lanes.
    let alphabet =
        _mm256_broadcastsi128_si256(_mm_loadu_si128(config.alphabet.as_ptr() as *const __m128i));
    let mask_0f = _mm256_set1_epi16(0x000F);

    // After packus_epi16(hi, lo) the per-lane layout is:
    //   [h0,h1,...,h7, l0,l1,...,l7]
    //
    // This shuffle interleaves to the correct output order:
    //   [h0,l0, h1,l1, ..., h7,l7]
    let interleave_shuf = _mm256_broadcastsi128_si256(_mm_set_epi8(
        15, 7, 14, 6, 13, 5, 12, 4, 11, 3, 10, 2, 9, 1, 8, 0,
    ));

    let mut src_offset = 0usize;
    let mut dst_offset = 0usize;

    while src_offset + 16 <= src.len() {
        let input = _mm_loadu_si128(src.as_ptr().add(src_offset) as *const __m128i);

        // Zero-extend 16 bytes to 16 × 16-bit words; isolates each byte so
        // srli_epi16 cannot leak bits from a neighbouring byte.
        let wide = _mm256_cvtepu8_epi16(input);

        let hi = _mm256_srli_epi16(wide, 4);
        let lo = _mm256_and_si256(wide, mask_0f);

        // Pack 16-bit lanes back to bytes (per-lane), then interleave hi/lo.
        let packed = _mm256_packus_epi16(hi, lo);
        let nibbles = _mm256_shuffle_epi8(packed, interleave_shuf);

        let chars = _mm256_shuffle_epi8(alphabet, nibbles);

        _mm256_storeu_si256(dst.as_mut_ptr().add(dst_offset) as *mut __m256i, chars);

        src_offset += 16;
        dst_offset += 32;
    }

    Ok(dst_offset)
}
