use core::arch::x86_64::*;

use crate::base64::config::Base64EncodeConfig;
use crate::base64::error::Base64Error;

#[inline(always)]
#[allow(unsafe_op_in_unsafe_fn)]
pub(crate) unsafe fn encode_full_groups_into_sse3(
    _config: &Base64EncodeConfig,
    dst: &mut [u8],
    src: &[u8],
) -> Result<usize, Base64Error> {
    let mut offset = 0usize;

    for chunk in src.chunks_exact(48) {
        let dst_ptr = dst.as_mut_ptr().add(offset);

        encode_sse3_block(chunk.as_ptr(), dst_ptr);

        offset += 64;
    }

    Ok(offset)
}

#[inline(always)]
#[allow(unsafe_op_in_unsafe_fn)]
unsafe fn encode_sse3_block(src: *const u8, dst: *mut u8) {
    let input0 = _mm_loadu_si128(src as *const __m128i);
    let input1 = _mm_loadu_si128(src.add(16) as *const __m128i);
    let input2 = _mm_loadu_si128(src.add(32) as *const __m128i);

    let mask = _mm_set1_epi8(0x3F);

    let b0 = _mm_and_si128(input0, mask);
    let b1 = _mm_and_si128(_mm_srli_epi16(input0, 2), mask);
    let _b2 = _mm_and_si128(_mm_srli_epi16(input0, 4), mask);
    let b3 = _mm_and_si128(_mm_srli_epi16(input0, 6), mask);

    let b4 = _mm_and_si128(input1, mask);
    let b5 = _mm_and_si128(_mm_srli_epi16(input1, 2), mask);
    let b6 = _mm_and_si128(_mm_srli_epi16(input1, 4), mask);
    let b7 = _mm_and_si128(_mm_srli_epi16(input1, 6), mask);

    let b8 = _mm_and_si128(input2, mask);
    let b9 = _mm_and_si128(_mm_srli_epi16(input2, 2), mask);
    let b10 = _mm_and_si128(_mm_srli_epi16(input2, 4), mask);
    let b11 = _mm_and_si128(_mm_srli_epi16(input2, 6), mask);

    let shift_mask_lo: __m128i = _mm_set_epi8(
        15i8, 15i8, 15i8, 15i8, 15i8, 15i8, 15i8, 15i8, 14i8, 13i8, 12i8, 11i8, 10i8, 9i8, 8i8, 7i8,
    );
    let shift_mask_hi: __m128i = _mm_set_epi8(
        6i8, 5i8, 4i8, 3i8, 2i8, 1i8, 0i8, -128i8, -128i8, -128i8, -128i8, -128i8, -128i8, -128i8,
        -128i8, -128i8,
    );

    let lo_shifted = _mm_shuffle_epi8(input0, shift_mask_lo);
    let hi_shifted = _mm_shuffle_epi8(input0, shift_mask_hi);

    let idx0 = _mm_or_si128(_mm_and_si128(lo_shifted, _mm_set1_epi8(0xC0u8 as i8)), b0);
    let idx1 = _mm_or_si128(_mm_and_si128(hi_shifted, _mm_set1_epi8(0x30u8 as i8)), b1);
    let idx2 = _mm_or_si128(lo_shifted, _mm_set1_epi8(0xC0u8 as i8));

    let alphabet_lo: __m128i = _mm_set_epi8(
        63i8, 62i8, 61i8, 60i8, 59i8, 58i8, 57i8, 56i8, 55i8, 54i8, 53i8, 52i8, 51i8, 50i8, 49i8,
        48i8,
    );
    let alphabet_hi: __m128i = _mm_set_epi8(
        47i8, 46i8, 45i8, 44i8, 43i8, 42i8, 41i8, 40i8, 39i8, 38i8, 37i8, 36i8, 35i8, 34i8, 33i8,
        32i8,
    );

    let char0 = _mm_shuffle_epi8(alphabet_lo, idx0);
    let char1 = _mm_shuffle_epi8(alphabet_hi, idx1);
    let char2 = _mm_shuffle_epi8(alphabet_lo, idx2);

    _mm_storeu_si128(dst as *mut __m128i, char0);
    _mm_storeu_si128(dst.add(16) as *mut __m128i, char1);
    _mm_storeu_si128(dst.add(32) as *mut __m128i, char2);

    let _ = (b3, b4, b5, b6, b7, b8, b9, b10, b11);
}
