use core::arch::x86_64::*;

use crate::base64::config::Base64EncodeConfig;
use crate::base64::error::Base64Error;

#[target_feature(enable = "avx2")]
#[inline]
pub(crate) fn avx2_encode_full_groups_into(
    config: &Base64EncodeConfig,
    dst: &mut [u8],
    src: &[u8],
) -> Result<usize, Base64Error> {
    let alphabet_ptr = config.alphabet.as_ptr();

    let shuf_mask = _mm_set_epi8(10, 11, 9, 10, 7, 8, 6, 7, 4, 5, 3, 4, 1, 2, 0, 1);
    let shuf_mask = _mm256_broadcastsi128_si256(shuf_mask);

    let mask0 = _mm256_set1_epi32(0x0FC0FC00_i32);
    let mask1 = _mm256_set1_epi32(0x003F03F0_i32);
    let mul_mask0 = _mm256_set1_epi32(0x04000040_i32);
    let mul_mask1 = _mm256_set1_epi32(0x01000010_i32);

    let (alpha0, alpha1, alpha2, alpha3) = unsafe {
        (
            _mm256_broadcastsi128_si256(_mm_loadu_si128(alphabet_ptr as *const __m128i)),
            _mm256_broadcastsi128_si256(_mm_loadu_si128(alphabet_ptr.add(16) as *const __m128i)),
            _mm256_broadcastsi128_si256(_mm_loadu_si128(alphabet_ptr.add(32) as *const __m128i)),
            _mm256_broadcastsi128_si256(_mm_loadu_si128(alphabet_ptr.add(48) as *const __m128i)),
        )
    };

    let sel_zero = _mm256_setzero_si256();
    let sel_one = _mm256_set1_epi8(0x10_i8);
    let sel_two = _mm256_set1_epi8(0x20_i8);
    let sel_three = _mm256_set1_epi8(0x30_i8);
    let idx_low_mask = _mm256_set1_epi8(0x0F_i8);

    let mut src_offset = 0usize;
    let mut dst_offset = 0usize;

    while src_offset + 24 <= src.len() {
        unsafe {
            let src_ptr = src.as_ptr().add(src_offset);
            let dst_ptr = dst.as_mut_ptr().add(dst_offset);
            avx2_encode_block(
                shuf_mask,
                mask0,
                mask1,
                mul_mask0,
                mul_mask1,
                alpha0,
                alpha1,
                alpha2,
                alpha3,
                sel_zero,
                sel_one,
                sel_two,
                sel_three,
                idx_low_mask,
                dst_ptr,
                src_ptr,
            );
        }

        src_offset += 24;
        dst_offset += 32;
    }

    Ok(dst_offset)
}

#[target_feature(enable = "avx2")]
#[inline]
#[allow(unsafe_op_in_unsafe_fn)]
unsafe fn avx2_encode_block(
    shuf_mask: __m256i,
    mask0: __m256i,
    mask1: __m256i,
    mul_mask0: __m256i,
    mul_mask1: __m256i,
    alpha0: __m256i,
    alpha1: __m256i,
    alpha2: __m256i,
    alpha3: __m256i,
    sel_zero: __m256i,
    sel_one: __m256i,
    sel_two: __m256i,
    sel_three: __m256i,
    idx_low_mask: __m256i,
    dst: *mut u8,
    src: *const u8,
) {
    let lo = _mm_loadu_si128(src as *const __m128i);
    let hi = _mm_loadu_si128(src.add(12) as *const __m128i);
    let input = _mm256_set_m128i(hi, lo);

    let shuffled = _mm256_shuffle_epi8(input, shuf_mask);

    let t0 = _mm256_mulhi_epu16(_mm256_and_si256(shuffled, mask0), mul_mask0);
    let t1 = _mm256_mullo_epi16(_mm256_and_si256(shuffled, mask1), mul_mask1);

    let indices = _mm256_or_si256(t0, t1);

    let upper2 = _mm256_and_si256(indices, _mm256_set1_epi8(0x30_i8));
    let sel0 = _mm256_cmpeq_epi8(upper2, sel_zero);
    let sel1 = _mm256_cmpeq_epi8(upper2, sel_one);
    let sel2 = _mm256_cmpeq_epi8(upper2, sel_two);
    let sel3 = _mm256_cmpeq_epi8(upper2, sel_three);

    let idx_low = _mm256_and_si256(indices, idx_low_mask);
    let r0 = _mm256_and_si256(_mm256_shuffle_epi8(alpha0, idx_low), sel0);
    let r1 = _mm256_and_si256(_mm256_shuffle_epi8(alpha1, idx_low), sel1);
    let r2 = _mm256_and_si256(_mm256_shuffle_epi8(alpha2, idx_low), sel2);
    let r3 = _mm256_and_si256(_mm256_shuffle_epi8(alpha3, idx_low), sel3);

    let result = _mm256_or_si256(_mm256_or_si256(r0, r1), _mm256_or_si256(r2, r3));

    _mm256_storeu_si256(dst as *mut __m256i, result);
}
