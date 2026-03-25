use core::arch::x86_64::*;

use crate::base64::config::Base64EncodeConfig;
use crate::base64::error::Base64Error;

/// Encodes full 48-byte groups (src.len() is guaranteed to be a multiple of 48).
/// Each 48 bytes → 64 base64 characters. Tail/padding is handled by the caller.
#[target_feature(enable = "avx512f,avx512bw")]
#[inline]
pub(crate) fn avx512_encode_full_groups_into(
    config: &Base64EncodeConfig,
    dst: &mut [u8],
    src: &[u8],
) -> Result<usize, Base64Error> {
    let alphabet_ptr = config.alphabet.as_ptr();
    let mut src_offset = 0usize;
    let mut dst_offset = 0usize;

    while src_offset + 48 <= src.len() {
        unsafe {
            let src_ptr = src.as_ptr().add(src_offset);
            let dst_ptr = dst.as_mut_ptr().add(dst_offset);
            avx512_encode_block(alphabet_ptr, dst_ptr, src_ptr);
        }

        src_offset += 48;
        dst_offset += 64;
    }

    Ok(dst_offset)
}

/// Encodes 48 src bytes into 64 base64 characters using the provided alphabet.
///
/// The 48 bytes form 16 complete triples; there is no padding.
/// Processes four 12-byte groups simultaneously in the four 128-bit lanes of a ZMM register.
///
/// # Loading strategy
///
/// `vpshufb` is lane-local, so a straight 64-byte load would misalign lanes 1–3.
/// Instead we do four 16-byte loads and combine with `_mm512_inserti32x4`:
///   l0 = loadu(src +  0) → lane 0: bytes  0–15 (bytes  0–11 used, 12–15 ignored by shuffle)
///   l1 = loadu(src + 12) → lane 1: bytes 12–27 (bytes 12–23 used, 24–27 ignored by shuffle)
///   l2 = loadu(src + 24) → lane 2: bytes 24–39 (bytes 24–35 used, 36–39 ignored by shuffle)
///   l3 = loadu(src + 36) → lane 3: bytes 36–51 (bytes 36–47 used, 48–51 ignored by shuffle)
///
/// After combining, each lane independently holds exactly the 12 bytes it needs.
///
/// # Algorithm (Muła, AVX-512 — mirrors the AVX2 version per-lane)
///
/// 1. Reshuffle: for each triple (a, b, c) → [b, a, c, b] as a 32-bit LE word.
/// 2. Extract 6-bit indices via masked multiply:
///      idx0 = (a >> 2) & 0x3F
///      idx1 = ((a & 0x03) << 4) | (b >> 4)
///      idx2 = ((b & 0x0F) << 2) | (c >> 6)
///      idx3 = c & 0x3F
/// 3. Map each index to the alphabet character with 4×vpshufb (covers all 64 chars).
///    Uses `_mm512_cmpeq_epi8_mask` + `_mm512_maskz_shuffle_epi8` because AVX-512
///    comparisons return __mmask64 rather than a full data register.
#[target_feature(enable = "avx512f,avx512bw")]
#[inline]
#[allow(unsafe_op_in_unsafe_fn)]
unsafe fn avx512_encode_block(alphabet: *const u8, dst: *mut u8, src: *const u8) {
    // Load four overlapping 16-byte windows so that each ZMM lane gets the 12
    // bytes it needs at positions 0–11 within that lane.
    let l0 = _mm_loadu_si128(src as *const __m128i);
    let l1 = _mm_loadu_si128(src.add(12) as *const __m128i);
    let l2 = _mm_loadu_si128(src.add(24) as *const __m128i);
    let l3 = _mm_loadu_si128(src.add(36) as *const __m128i);

    // Combine: l0 → lane 0, l1 → lane 1, l2 → lane 2, l3 → lane 3.
    let input = _mm512_broadcast_i32x4(l0);
    let input = _mm512_inserti32x4(input, l1, 1);
    let input = _mm512_inserti32x4(input, l2, 2);
    let input = _mm512_inserti32x4(input, l3, 3);

    // Step 1 — Reshuffle (lane-local vpshufb).
    //
    // Each lane input: [ a0  b0  c0 | a1  b1  c1 | a2  b2  c2 | a3  b3  c3  ?  ?  ?  ? ]
    // Each lane →      [ b  a  c  b | b  a  c  b | b  a  c  b | b  a  c  b ]
    //
    // The same 128-bit mask is broadcast to all four lanes.
    let shuf_mask = _mm_set_epi8(10, 11, 9, 10, 7, 8, 6, 7, 4, 5, 3, 4, 1, 2, 0, 1);
    let shuffled = _mm512_shuffle_epi8(input, _mm512_broadcast_i32x4(shuf_mask));

    // Step 2 — Extract 6-bit indices.
    //
    // Identical constants to the AVX2 implementation, broadcast across all four lanes.
    let mask0 = _mm512_set1_epi32(0x0FC0FC00u32 as i32);
    let mask1 = _mm512_set1_epi32(0x003F03F0u32 as i32);

    let t0 = _mm512_mulhi_epu16(
        _mm512_and_si512(shuffled, mask0),
        _mm512_set1_epi32(0x04000040u32 as i32),
    );
    let t1 = _mm512_mullo_epi16(
        _mm512_and_si512(shuffled, mask1),
        _mm512_set1_epi32(0x01000010u32 as i32),
    );

    // indices[i] ∈ [0, 63] for each byte i
    let indices = _mm512_or_si512(t0, t1);

    // Step 3 — Alphabet lookup for an arbitrary 64-byte alphabet.
    //
    // Each 16-char chunk of the alphabet is broadcast to all four lanes.
    // AVX-512 cmpeq returns __mmask64 (one bit per byte) instead of a data
    // register, so we use _mm512_maskz_shuffle_epi8 for the zero-masking step.
    //
    //   bits 5..4 of index → which block:  00 → alpha0,  01 → alpha1,
    //                                       10 → alpha2,  11 → alpha3
    let alpha0 = _mm512_broadcast_i32x4(_mm_loadu_si128(alphabet as *const __m128i));
    let alpha1 = _mm512_broadcast_i32x4(_mm_loadu_si128(alphabet.add(16) as *const __m128i));
    let alpha2 = _mm512_broadcast_i32x4(_mm_loadu_si128(alphabet.add(32) as *const __m128i));
    let alpha3 = _mm512_broadcast_i32x4(_mm_loadu_si128(alphabet.add(48) as *const __m128i));

    let upper2 = _mm512_and_si512(indices, _mm512_set1_epi8(0x30u8 as i8));
    let sel0 = _mm512_cmpeq_epi8_mask(upper2, _mm512_setzero_si512());
    let sel1 = _mm512_cmpeq_epi8_mask(upper2, _mm512_set1_epi8(0x10u8 as i8));
    let sel2 = _mm512_cmpeq_epi8_mask(upper2, _mm512_set1_epi8(0x20u8 as i8));
    let sel3 = _mm512_cmpeq_epi8_mask(upper2, _mm512_set1_epi8(0x30u8 as i8));

    let idx_low = _mm512_and_si512(indices, _mm512_set1_epi8(0x0F));
    let r0 = _mm512_maskz_shuffle_epi8(sel0, alpha0, idx_low);
    let r1 = _mm512_maskz_shuffle_epi8(sel1, alpha1, idx_low);
    let r2 = _mm512_maskz_shuffle_epi8(sel2, alpha2, idx_low);
    let r3 = _mm512_maskz_shuffle_epi8(sel3, alpha3, idx_low);

    let result = _mm512_or_si512(_mm512_or_si512(r0, r1), _mm512_or_si512(r2, r3));

    _mm512_storeu_si512(dst as *mut __m512i, result);
}
