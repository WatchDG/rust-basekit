use core::arch::x86_64::*;

use crate::base16::config::Base16EncodeConfig;
use crate::base16::error::Base16Error;

/// Encodes bytes into base16 (hex) characters using SSSE3.
///
/// Processes 8 input bytes → 16 output chars per iteration.
/// Any remaining bytes after the SIMD loop are left to the caller.
///
/// Returns the number of output bytes written to `dst`.
#[target_feature(enable = "ssse3")]
#[inline]
#[allow(unsafe_op_in_unsafe_fn)]
pub(crate) unsafe fn ssse3_encode_into(
    config: &Base16EncodeConfig,
    dst: &mut [u8],
    src: &[u8],
) -> Result<usize, Base16Error> {
    // The 16-char alphabet fits entirely in a single 128-bit register,
    // so a single pshufb can map any nibble (0–15) to its hex character.
    let alphabet = _mm_loadu_si128(config.alphabet.as_ptr() as *const __m128i);
    let mask_0f = _mm_set1_epi8(0x0F);

    let mut src_offset = 0usize;
    let mut dst_offset = 0usize;

    while src_offset + 8 <= src.len() {
        let input = _mm_loadl_epi64(src.as_ptr().add(src_offset) as *const __m128i);

        let hi = _mm_and_si128(_mm_srli_epi16(input, 4), mask_0f);
        let lo = _mm_and_si128(input, mask_0f);

        // Interleave high and low nibbles: [h0,l0, h1,l1, ..., h7,l7].
        let nibbles = _mm_unpacklo_epi8(hi, lo);

        let chars = _mm_shuffle_epi8(alphabet, nibbles);

        _mm_storeu_si128(dst.as_mut_ptr().add(dst_offset) as *mut __m128i, chars);

        src_offset += 8;
        dst_offset += 16;
    }

    Ok(dst_offset)
}
