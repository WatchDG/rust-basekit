use core::arch::x86_64::*;

use crate::base64::config::Base64EncodeConfig;
use crate::base64::error::Base64Error;

/// Encodes full 24-byte groups (src.len() is guaranteed to be a multiple of 24).
/// Each 24 bytes → 32 base64 characters. Tail/padding is handled by the caller.
#[target_feature(enable = "avx2")]
#[inline]
#[allow(unsafe_op_in_unsafe_fn)]
pub(crate) unsafe fn avx2_encode_full_groups_into(
    config: &Base64EncodeConfig,
    dst: &mut [u8],
    src: &[u8],
) -> Result<usize, Base64Error> {
    let alphabet_ptr = config.alphabet.as_ptr();
    let mut src_offset = 0usize;
    let mut dst_offset = 0usize;

    while src_offset + 24 <= src.len() {
        let src_ptr = src.as_ptr().add(src_offset);
        let dst_ptr = dst.as_mut_ptr().add(dst_offset);

        encode_avx2_block(src_ptr, dst_ptr, alphabet_ptr);

        src_offset += 24;
        dst_offset += 32;
    }

    Ok(dst_offset)
}

/// Encodes 24 src bytes into 32 base64 characters using the provided alphabet.
///
/// The 24 bytes form 8 complete triples; there is no padding.
/// Processes two 12-byte groups simultaneously in the two 128-bit lanes of a YMM register.
///
/// # Loading strategy
///
/// `vpshufb` is lane-local, so a straight 32-byte load would give:
///   lane 0: bytes  0–15  (12 valid + 4 overlap)
///   lane 1: bytes 16–31  (only 8 of the 12 valid bytes belong to this lane)
///
/// Instead we do two 16-byte loads and combine with `_mm256_set_m128i`:
///   lo = loadu(src +  0) → lane 0: bytes  0–15 (bytes  0–11 used, 12–15 ignored by shuffle)
///   hi = loadu(src + 12) → lane 1: bytes 12–27 (bytes 12–23 used, 24–27 ignored by shuffle)
///
/// After combining, each lane independently holds exactly the 12 bytes it needs.
///
/// # Algorithm (Muła, AVX2 — mirrors the SSSE3 version per-lane)
///
/// 1. Reshuffle: for each triple (a, b, c) → [b, a, c, b] as a 32-bit LE word.
/// 2. Extract 6-bit indices via masked multiply:
///      idx0 = (a >> 2) & 0x3F
///      idx1 = ((a & 0x03) << 4) | (b >> 4)
///      idx2 = ((b & 0x0F) << 2) | (c >> 6)
///      idx3 = c & 0x3F
/// 3. Map each index to the alphabet character with 4×vpshufb (covers all 64 chars).
#[target_feature(enable = "avx2")]
#[inline]
#[allow(unsafe_op_in_unsafe_fn)]
unsafe fn encode_avx2_block(src: *const u8, dst: *mut u8, alphabet: *const u8) {
    // Load two overlapping 16-byte windows so that each YMM lane gets the 12
    // bytes it needs at positions 0–11 within that lane.
    let lo = _mm_loadu_si128(src as *const __m128i);
    let hi = _mm_loadu_si128(src.add(12) as *const __m128i);
    let input = _mm256_set_m128i(hi, lo);

    // Step 1 — Reshuffle (lane-local vpshufb).
    //
    // Lane 0 input: [ a0  b0  c0 | a1  b1  c1 | a2  b2  c2 | a3  b3  c3  ?  ?  ?  ? ]
    // Lane 1 input: [ a4  b4  c4 | a5  b5  c5 | a6  b6  c6 | a7  b7  c7  ?  ?  ?  ? ]
    // Each lane → [ b  a  c  b | b  a  c  b | b  a  c  b | b  a  c  b ]
    //
    // The same 128-bit mask is broadcast to both lanes.
    let shuf_mask = _mm_set_epi8(10, 11, 9, 10, 7, 8, 6, 7, 4, 5, 3, 4, 1, 2, 0, 1);
    let shuffled = _mm256_shuffle_epi8(input, _mm256_broadcastsi128_si256(shuf_mask));

    // Step 2 — Extract 6-bit indices.
    //
    // Identical constants to the SSSE3 implementation, broadcast across both lanes.
    let mask0 = _mm256_set1_epi32(0x0FC0FC00u32 as i32);
    let mask1 = _mm256_set1_epi32(0x003F03F0u32 as i32);

    let t0 = _mm256_mulhi_epu16(
        _mm256_and_si256(shuffled, mask0),
        _mm256_set1_epi32(0x04000040u32 as i32),
    );
    let t1 = _mm256_mullo_epi16(
        _mm256_and_si256(shuffled, mask1),
        _mm256_set1_epi32(0x01000010u32 as i32),
    );

    // indices[i] ∈ [0, 63] for each byte i
    let indices = _mm256_or_si256(t0, t1);

    // Step 3 — Alphabet lookup for an arbitrary 64-byte alphabet.
    //
    // Same 4-block strategy as SSSE3: each 16-char chunk of the alphabet is
    // broadcast to both lanes, selected by comparing bits 5..4 of the index.
    let alpha0 = _mm256_broadcastsi128_si256(_mm_loadu_si128(alphabet as *const __m128i));
    let alpha1 = _mm256_broadcastsi128_si256(_mm_loadu_si128(alphabet.add(16) as *const __m128i));
    let alpha2 = _mm256_broadcastsi128_si256(_mm_loadu_si128(alphabet.add(32) as *const __m128i));
    let alpha3 = _mm256_broadcastsi128_si256(_mm_loadu_si128(alphabet.add(48) as *const __m128i));

    let upper2 = _mm256_and_si256(indices, _mm256_set1_epi8(0x30u8 as i8));
    let sel0 = _mm256_cmpeq_epi8(upper2, _mm256_setzero_si256());
    let sel1 = _mm256_cmpeq_epi8(upper2, _mm256_set1_epi8(0x10u8 as i8));
    let sel2 = _mm256_cmpeq_epi8(upper2, _mm256_set1_epi8(0x20u8 as i8));
    let sel3 = _mm256_cmpeq_epi8(upper2, _mm256_set1_epi8(0x30u8 as i8));

    let idx_low = _mm256_and_si256(indices, _mm256_set1_epi8(0x0F));
    let r0 = _mm256_and_si256(_mm256_shuffle_epi8(alpha0, idx_low), sel0);
    let r1 = _mm256_and_si256(_mm256_shuffle_epi8(alpha1, idx_low), sel1);
    let r2 = _mm256_and_si256(_mm256_shuffle_epi8(alpha2, idx_low), sel2);
    let r3 = _mm256_and_si256(_mm256_shuffle_epi8(alpha3, idx_low), sel3);

    let result = _mm256_or_si256(_mm256_or_si256(r0, r1), _mm256_or_si256(r2, r3));

    _mm256_storeu_si256(dst as *mut __m256i, result);
}
