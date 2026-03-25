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
    let mut src_offset = 0usize;
    let mut dst_offset = 0usize;

    while src_offset + 12 <= src.len() {
        unsafe {
            let src_ptr = src.as_ptr().add(src_offset);
            let dst_ptr = dst.as_mut_ptr().add(dst_offset);
            ssse3_encode_block(alphabet_ptr, dst_ptr, src_ptr);
        }

        src_offset += 12;
        dst_offset += 16;
    }

    Ok(dst_offset)
}

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
unsafe fn ssse3_encode_block(alphabet: *const u8, dst: *mut u8, src: *const u8) {
    // Load 16 bytes (first 12 are the 4 input triples, last 4 are not used)
    let input = _mm_loadu_si128(src as *const __m128i);

    // Step 1 — Reshuffle.
    //
    // Input:    [ a0  b0  c0 | a1  b1  c1 | a2  b2  c2 | a3  b3  c3  ?  ?  ?  ? ]
    // Shuffled: [ b0  a0  c0  b0 | b1  a1  c1  b1 | b2  a2  c2  b2 | b3  a3  c3  b3 ]
    //
    // _mm_set_epi8 arguments: (e15, e14, ..., e1, e0) where eN is byte N.
    let shuffled = _mm_shuffle_epi8(
        input,
        _mm_set_epi8(10, 11, 9, 10, 7, 8, 6, 7, 4, 5, 3, 4, 1, 2, 0, 1),
    );

    // Step 2 — Extract 6-bit indices.
    //
    // Each 32-bit LE word is [b, a, c, b].  Two 16-bit views:
    //   lower word = b | (a << 8)
    //   upper word = c | (b << 8)
    //
    // mask0 picks the bits needed by mulhi:
    //   lower & 0xFC00 = (a & 0xFC) << 8  →  mulhi ×0x0040 → idx0 = a >> 2
    //   upper & 0x0FC0 = (b & 0x0F) << 8 | (c & 0xC0)  →  mulhi ×0x0400 → idx2
    //
    // mask1 picks the bits needed by mullo:
    //   lower & 0x03F0 = (a & 0x03) << 8 | (b & 0xF0)  →  mullo ×0x0010 → idx1 (high byte)
    //   upper & 0x003F = c & 0x3F                        →  mullo ×0x0100 → idx3 (high byte)
    let mask0 = _mm_set1_epi32(0x0FC0FC00u32 as i32);
    let mask1 = _mm_set1_epi32(0x003F03F0u32 as i32);

    let t0 = _mm_mulhi_epu16(
        _mm_and_si128(shuffled, mask0),
        _mm_set1_epi32(0x04000040u32 as i32),
    );
    let t1 = _mm_mullo_epi16(
        _mm_and_si128(shuffled, mask1),
        _mm_set1_epi32(0x01000010u32 as i32),
    );

    // indices[i] ∈ [0, 63] for each byte i
    let indices = _mm_or_si128(t0, t1);

    // Step 3 — Alphabet lookup for an arbitrary 64-byte alphabet.
    //
    // pshufb zeros a lane when the index byte has bit 7 set, so a direct lookup
    // only works for indices 0..15.  We split the 64-char alphabet into four
    // 16-char blocks, select each via a comparison mask, then OR the results.
    //
    //   bits 5..4 of index → which block:  00 → alpha0,  01 → alpha1,
    //                                       10 → alpha2,  11 → alpha3
    let alpha0 = _mm_loadu_si128(alphabet as *const __m128i);
    let alpha1 = _mm_loadu_si128(alphabet.add(16) as *const __m128i);
    let alpha2 = _mm_loadu_si128(alphabet.add(32) as *const __m128i);
    let alpha3 = _mm_loadu_si128(alphabet.add(48) as *const __m128i);

    let upper2 = _mm_and_si128(indices, _mm_set1_epi8(0x30u8 as i8));
    let sel0 = _mm_cmpeq_epi8(upper2, _mm_setzero_si128());
    let sel1 = _mm_cmpeq_epi8(upper2, _mm_set1_epi8(0x10u8 as i8));
    let sel2 = _mm_cmpeq_epi8(upper2, _mm_set1_epi8(0x20u8 as i8));
    let sel3 = _mm_cmpeq_epi8(upper2, _mm_set1_epi8(0x30u8 as i8));

    let idx_low = _mm_and_si128(indices, _mm_set1_epi8(0x0F));
    let r0 = _mm_and_si128(_mm_shuffle_epi8(alpha0, idx_low), sel0);
    let r1 = _mm_and_si128(_mm_shuffle_epi8(alpha1, idx_low), sel1);
    let r2 = _mm_and_si128(_mm_shuffle_epi8(alpha2, idx_low), sel2);
    let r3 = _mm_and_si128(_mm_shuffle_epi8(alpha3, idx_low), sel3);

    let result = _mm_or_si128(_mm_or_si128(r0, r1), _mm_or_si128(r2, r3));

    _mm_storeu_si128(dst as *mut __m128i, result);
}
