use core::arch::x86_64::*;

use crate::base64::config::Base64EncodeConfig;
use crate::base64::error::Base64Error;

/// Encodes full 12-byte groups (src.len() is guaranteed to be a multiple of 12).
/// Each 12 bytes → 16 base64 characters. Tail/padding is handled by the caller.
#[target_feature(enable = "ssse3")]
#[inline]
pub(crate) fn ssse3_encode_full_groups_into(
    config: &Base64EncodeConfig,
    dst: &mut [u8],
    src: &[u8],
) -> Result<usize, Base64Error> {
    let alphabet_ptr = config.alphabet.as_ptr();

    let shuf_mask = _mm_set_epi8(10, 11, 9, 10, 7, 8, 6, 7, 4, 5, 3, 4, 1, 2, 0, 1);
    let mask0 = _mm_set1_epi32(0x0FC0FC00u32 as i32);
    let mask1 = _mm_set1_epi32(0x003F03F0u32 as i32);
    let mul_mask0 = _mm_set1_epi32(0x04000040u32 as i32);
    let mul_mask1 = _mm_set1_epi32(0x01000010u32 as i32);
    let mask_upper2 = _mm_set1_epi8(0x30u8 as i8);
    let sel1_mask = _mm_set1_epi8(0x10u8 as i8);
    let sel2_mask = _mm_set1_epi8(0x20u8 as i8);
    let mask_low4 = _mm_set1_epi8(0x0F);

    let (alpha0, alpha1, alpha2, alpha3) = unsafe {
        (
            _mm_loadu_si128(alphabet_ptr as *const __m128i),
            _mm_loadu_si128(alphabet_ptr.add(16) as *const __m128i),
            _mm_loadu_si128(alphabet_ptr.add(32) as *const __m128i),
            _mm_loadu_si128(alphabet_ptr.add(48) as *const __m128i),
        )
    };

    let mut src_offset = 0usize;
    let mut dst_offset = 0usize;

    while src_offset + 12 <= src.len() {
        unsafe {
            let src_ptr = src.as_ptr().add(src_offset);
            let dst_ptr = dst.as_mut_ptr().add(dst_offset);
            ssse3_encode_block(
                dst_ptr,
                src_ptr,
                shuf_mask,
                mask0,
                mask1,
                mul_mask0,
                mul_mask1,
                mask_upper2,
                sel1_mask,
                sel2_mask,
                mask_low4,
                alpha0,
                alpha1,
                alpha2,
                alpha3,
            );
        }

        src_offset += 12;
        dst_offset += 16;
    }

    Ok(dst_offset)
}

#[allow(clippy::doc_overindented_list_items)]
/// Encodes 12 src bytes into 16 base64 characters using the provided alphabet.
///
/// The 12 bytes form 4 complete triples; there is no padding.
/// Reads up to 16 bytes from `src` (last 4 are ignored), writes exactly 16 bytes to `dst`.
///
/// Algorithm (Muła, SSSE3):
///
/// 1. Reshuffle: for each triple (a, b, c) → [b, a, c, b] as a 32-bit LE word.
/// 2. Extract 6-bit indices via masked multiply:
///      idx0 = (a >> 2) & 0x3F
///      idx1 = ((a & 0x03) << 4) | (b >> 4)
///      idx2 = ((b & 0x0F) << 2) | (c >> 6)
///      idx3 = c & 0x3F
/// 3. Map each index to the alphabet character with 4×pshufb (covers all 64 chars).
#[target_feature(enable = "ssse3")]
#[inline]
#[allow(unsafe_op_in_unsafe_fn)]
unsafe fn ssse3_encode_block(
    dst: *mut u8,
    src: *const u8,
    shuf_mask: __m128i,
    mask0: __m128i,
    mask1: __m128i,
    mul_mask0: __m128i,
    mul_mask1: __m128i,
    mask_upper2: __m128i,
    sel1_mask: __m128i,
    sel2_mask: __m128i,
    mask_low4: __m128i,
    alpha0: __m128i,
    alpha1: __m128i,
    alpha2: __m128i,
    alpha3: __m128i,
) {
    let input = _mm_loadu_si128(src as *const __m128i);

    let shuffled = _mm_shuffle_epi8(input, shuf_mask);

    let t0 = _mm_mulhi_epu16(_mm_and_si128(shuffled, mask0), mul_mask0);
    let t1 = _mm_mullo_epi16(_mm_and_si128(shuffled, mask1), mul_mask1);

    let indices = _mm_or_si128(t0, t1);

    let upper2 = _mm_and_si128(indices, mask_upper2);
    let sel0 = _mm_cmpeq_epi8(upper2, _mm_setzero_si128());
    let sel1 = _mm_cmpeq_epi8(upper2, sel1_mask);
    let sel2 = _mm_cmpeq_epi8(upper2, sel2_mask);
    let sel3 = _mm_cmpeq_epi8(upper2, mask_upper2);

    let idx_low = _mm_and_si128(indices, mask_low4);
    let r0 = _mm_and_si128(_mm_shuffle_epi8(alpha0, idx_low), sel0);
    let r1 = _mm_and_si128(_mm_shuffle_epi8(alpha1, idx_low), sel1);
    let r2 = _mm_and_si128(_mm_shuffle_epi8(alpha2, idx_low), sel2);
    let r3 = _mm_and_si128(_mm_shuffle_epi8(alpha3, idx_low), sel3);

    let result = _mm_or_si128(_mm_or_si128(r0, r1), _mm_or_si128(r2, r3));

    _mm_storeu_si128(dst as *mut __m128i, result);
}
