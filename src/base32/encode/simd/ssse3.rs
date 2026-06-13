#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

use crate::base32::config::Base32EncodeConfig;
use crate::base32::error::Base32Error;

/// Encodes full 5-byte groups into base32 characters using SSSE3.
///
/// Processes 2 groups (10 input bytes → 16 output chars) per iteration.
/// The caller must ensure `src.len()` is a multiple of 5.
/// Any remaining group after the SIMD loop is left to the caller.
///
/// Returns the number of output bytes written to `dst`.
#[target_feature(enable = "ssse3")]
#[inline]
#[allow(unsafe_op_in_unsafe_fn)]
pub(crate) unsafe fn ssse3_encode_full_groups_into(
    config: &Base32EncodeConfig,
    dst: &mut [u8],
    src: &[u8],
) -> Result<usize, Base32Error> {
    debug_assert_eq!(src.len() % 5, 0);

    // Load the two 16-byte halves of the 32-char alphabet.
    let alphabet_ptr = config.alphabet.as_ptr();
    let lo_table = _mm_loadu_si128(alphabet_ptr as *const __m128i);
    let hi_table = _mm_loadu_si128(alphabet_ptr.add(16) as *const __m128i);

    // pshufb shuffle that builds eight 16-bit BE windows from the 10 input bytes.
    //
    // For 2 groups [a₀,b₀,c₀,d₀,e₀, a₁,b₁,c₁,d₁,e₁] (indices 0-9 in the 16-byte load):
    //   Lane 0: BE word (a₀,b₀) → stored LE as [b₀, a₀] at bytes (0,1)
    //   Lane 1: BE word (b₀,c₀) → stored LE as [c₀, b₀] at bytes (2,3)
    //   Lane 2: BE word (c₀,d₀) → stored LE as [d₀, c₀] at bytes (4,5)
    //   Lane 3: BE word (d₀,e₀) → stored LE as [e₀, d₀] at bytes (6,7)
    //   Lane 4: BE word (a₁,b₁) → stored LE as [b₁, a₁] at bytes (8,9)
    //   Lane 5: BE word (b₁,c₁) → stored LE as [c₁, b₁] at bytes (10,11)
    //   Lane 6: BE word (c₁,d₁) → stored LE as [d₁, c₁] at bytes (12,13)
    //   Lane 7: BE word (d₁,e₁) → stored LE as [e₁, d₁] at bytes (14,15)
    //
    // _mm_set_epi8(e15,...,e0): e_i is written to output byte i.
    // pshufb output[i] = input[shuf[i]], so shuf = [1,0,2,1,3,2,4,3,6,5,7,6,8,7,9,8].
    let spread_shuf = _mm_set_epi8(8, 9, 7, 8, 6, 7, 5, 6, 3, 4, 2, 3, 1, 2, 0, 1);

    // mulhi_epu16(v, k) = (v * k) >> 16 = v >> (16 - log2(k))
    //
    // "First" 5-bit index per window — right-shifts 11, 9, 7, 5:
    //   Lane 0 (shift 11): k = 2^5  = 32
    //   Lane 1 (shift  9): k = 2^7  = 128
    //   Lane 2 (shift  7): k = 2^9  = 512
    //   Lane 3 (shift  5): k = 2^11 = 2048   (& 0x1F removes overflow bits)
    //
    // _mm_set_epi16(e7,...,e0): e_i is the multiplier for lane i.
    let mul_first = _mm_set_epi16(2048, 512, 128, 32, 2048, 512, 128, 32);

    // "Second" 5-bit index per window — right-shifts 6, 4, 2, 0:
    //   Lane 0 (shift 6): k = 2^10 = 1024
    //   Lane 1 (shift 4): k = 2^12 = 4096
    //   Lane 2 (shift 2): k = 2^14 = 16384
    //   Lane 3 (shift 0): k = 0 → mulhi gives 0; handled separately via bitwise AND.
    let mul_second = _mm_set_epi16(0, 16384, 4096, 1024, 0, 16384, 4096, 1024);

    // Mask for lanes 3 and 7 (shift = 0): c₇ = e_byte & 31 = (d*256+e) & 0x1F.
    let mask_c7 = _mm_set_epi16(0x001F, 0, 0, 0, 0x001F, 0, 0, 0);

    let mask_5bit = _mm_set1_epi16(0x001F);

    // After packus_epi16(first_idx, second_idx) the byte layout is:
    //   [c₀,c₂,c₄,c₆, c₀₁,c₂₁,c₄₁,c₆₁ | c₁,c₃,c₅,c₇, c₁₁,c₃₁,c₅₁,c₇₁]
    //
    // We interleave to get the correct output order:
    //   [c₀,c₁,c₂,c₃,c₄,c₅,c₆,c₇, c₀₁,c₁₁,c₂₁,c₃₁,c₄₁,c₅₁,c₆₁,c₇₁]
    //
    // Shuffle: output[i] ← input[interleave_shuf[i]]
    //   [0,8, 1,9, 2,10, 3,11, 4,12, 5,13, 6,14, 7,15]
    let interleave_shuf = _mm_set_epi8(15, 7, 14, 6, 13, 5, 12, 4, 11, 3, 10, 2, 9, 1, 8, 0);

    let mut src_offset = 0usize;
    let mut dst_offset = 0usize;

    // Process 2 groups (10 src bytes → 16 dst bytes) per iteration.
    // The guard `src_offset + 16 <= src.len()` ensures the 16-byte loadu is always safe.
    while src_offset + 16 <= src.len() {
        let input = _mm_loadu_si128(src.as_ptr().add(src_offset) as *const __m128i);

        // Build 8 × 16-bit BE windows via byte shuffle.
        let windows = _mm_shuffle_epi8(input, spread_shuf);

        // Extract "first" indices: c₀,c₂,c₄,c₆ (even positions) for both groups.
        let first_idx = _mm_and_si128(_mm_mulhi_epu16(windows, mul_first), mask_5bit);

        // Extract "second" indices: c₁,c₃,c₅ via mulhi; c₇ via direct mask.
        let second_a = _mm_and_si128(_mm_mulhi_epu16(windows, mul_second), mask_5bit);
        let c7_vals = _mm_and_si128(windows, mask_c7);
        let second_idx = _mm_or_si128(second_a, c7_vals);

        // Pack 16-bit lanes to bytes, then interleave even/odd indices.
        let packed = _mm_shuffle_epi8(_mm_packus_epi16(first_idx, second_idx), interleave_shuf);

        // 32-entry alphabet lookup using two pshufb passes.
        //
        // lo_idx: for i in 0..16: i + 0x70 ∈ [0x70,0x7F] → bit7=0, pshufb uses low nibble = i ✓
        //         for i in 16..32: i + 0x70 ∈ [0x80,0x8F] → bit7=1, pshufb outputs 0 ✓
        //
        // hi_idx: for i in 0..16: (i as i8) - 16 < 0 → bit7=1, pshufb outputs 0 ✓
        //         for i in 16..32: i - 16 ∈ [0,15] → bit7=0, pshufb uses i-16 ✓
        let lo_idx = _mm_add_epi8(packed, _mm_set1_epi8(0x70u8 as i8));
        let hi_idx = _mm_sub_epi8(packed, _mm_set1_epi8(16));
        let chars = _mm_or_si128(
            _mm_shuffle_epi8(lo_table, lo_idx),
            _mm_shuffle_epi8(hi_table, hi_idx),
        );

        _mm_storeu_si128(dst.as_mut_ptr().add(dst_offset) as *mut __m128i, chars);

        src_offset += 10;
        dst_offset += 16;
    }

    Ok(dst_offset)
}
