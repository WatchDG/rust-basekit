use core::arch::x86_64::*;

use crate::base32::config::Base32EncodeConfig;
use crate::base32::error::Base32Error;

/// Encodes full 5-byte groups into base32 characters using AVX2.
///
/// Processes 4 groups (20 input bytes → 32 output chars) per iteration.
/// The caller must ensure `src.len()` is a multiple of 5.
/// Any remaining groups after the SIMD loop are left to the caller.
///
/// Returns the number of output bytes written to `dst`.
#[target_feature(enable = "avx2")]
#[inline]
#[allow(unsafe_op_in_unsafe_fn)]
pub(crate) unsafe fn avx2_encode_full_groups_into(
    config: &Base32EncodeConfig,
    dst: &mut [u8],
    src: &[u8],
) -> Result<usize, Base32Error> {
    debug_assert_eq!(src.len() % 5, 0);

    let alphabet_ptr = config.alphabet.as_ptr();

    // Broadcast each 16-byte half of the 32-char alphabet to both 128-bit lanes.
    let lo_table = _mm256_broadcastsi128_si256(_mm_loadu_si128(alphabet_ptr as *const __m128i));
    let hi_table =
        _mm256_broadcastsi128_si256(_mm_loadu_si128(alphabet_ptr.add(16) as *const __m128i));

    // pshufb shuffle — same as SSSE3, broadcast to both lanes.
    //
    // Builds eight 16-bit BE windows from 10 input bytes per lane:
    //   [b,a], [c,b], [d,c], [e,d], [b',a'], [c',b'], [d',c'], [e',d']
    // for the two 5-byte groups present in each lane.
    let spread_shuf =
        _mm256_broadcastsi128_si256(_mm_set_epi8(8, 9, 7, 8, 6, 7, 5, 6, 3, 4, 2, 3, 1, 2, 0, 1));

    // Multipliers for extracting "first" (even-position) 5-bit indices via mulhi_epu16.
    //   Lane i shifts right by (11, 9, 7, 5) for windows (0,1,2,3) in each group.
    let mul_first =
        _mm256_broadcastsi128_si256(_mm_set_epi16(2048, 512, 128, 32, 2048, 512, 128, 32));

    // Multipliers for extracting "second" (odd-position) 5-bit indices via mulhi_epu16.
    //   Lane 3 (shift = 0) is handled separately via direct mask.
    let mul_second =
        _mm256_broadcastsi128_si256(_mm_set_epi16(0, 16384, 4096, 1024, 0, 16384, 4096, 1024));

    // Direct mask for lanes 3 and 7: c₇ = last_byte & 0x1F.
    let mask_c7 = _mm256_broadcastsi128_si256(_mm_set_epi16(0x001F, 0, 0, 0, 0x001F, 0, 0, 0));

    let mask_5bit = _mm256_set1_epi16(0x001F);

    // After packus_epi16(first_idx, second_idx) the byte layout per lane is:
    //   [c0,c2,c4,c6, c0',c2',c4',c6' | c1,c3,c5,c7, c1',c3',c5',c7']
    //
    // This shuffle interleaves to the correct output order:
    //   [c0,c1,c2,c3,c4,c5,c6,c7, c0',c1',c2',c3',c4',c5',c6',c7']
    let interleave_shuf = _mm256_broadcastsi128_si256(_mm_set_epi8(
        15, 7, 14, 6, 13, 5, 12, 4, 11, 3, 10, 2, 9, 1, 8, 0,
    ));

    let mut src_offset = 0usize;
    let mut dst_offset = 0usize;

    // Process 4 groups (20 src bytes → 32 dst bytes) per iteration.
    //
    // Loading strategy: vpshufb is lane-local so a straight 32-byte load does not work.
    // Two overlapping 16-byte loads are combined with _mm256_set_m128i:
    //   lo = loadu(src + src_offset)      → lane 0: groups 0 and 1 (bytes 0–9 used)
    //   hi = loadu(src + src_offset + 10) → lane 1: groups 2 and 3 (bytes 0–9 of this window)
    //
    // The second load touches bytes [src_offset+10 .. src_offset+26], so the guard is +26.
    while src_offset + 26 <= src.len() {
        let lo = _mm_loadu_si128(src.as_ptr().add(src_offset) as *const __m128i);
        let hi = _mm_loadu_si128(src.as_ptr().add(src_offset + 10) as *const __m128i);

        // hi → upper 128-bit lane, lo → lower 128-bit lane.
        let input = _mm256_set_m128i(hi, lo);

        // Build 8 × 16-bit BE windows per lane via byte shuffle.
        let windows = _mm256_shuffle_epi8(input, spread_shuf);

        // Extract "first" indices: c0,c2,c4,c6 (and c0',c2',c4',c6') for both lanes.
        let first_idx = _mm256_and_si256(_mm256_mulhi_epu16(windows, mul_first), mask_5bit);

        // Extract "second" indices: c1,c3,c5 via mulhi; c7 via direct mask.
        let second_a = _mm256_and_si256(_mm256_mulhi_epu16(windows, mul_second), mask_5bit);
        let c7_vals = _mm256_and_si256(windows, mask_c7);
        let second_idx = _mm256_or_si256(second_a, c7_vals);

        // Pack 16-bit lanes to bytes (per-lane), then interleave even/odd indices.
        let packed =
            _mm256_shuffle_epi8(_mm256_packus_epi16(first_idx, second_idx), interleave_shuf);

        // 32-entry alphabet lookup using two vpshufb passes.
        //
        // lo_idx: indices 0..15 → lo_idx + 0x70 ∈ [0x70,0x7F] → bit7=0, pshufb uses low nibble ✓
        //         indices 16..31 → lo_idx + 0x70 ∈ [0x80,0x8F] → bit7=1, pshufb outputs 0   ✓
        //
        // hi_idx: indices 0..15 → hi_idx - 16 < 0 → bit7=1, pshufb outputs 0               ✓
        //         indices 16..31 → hi_idx - 16 ∈ [0,15] → bit7=0, pshufb uses low nibble    ✓
        let lo_idx = _mm256_add_epi8(packed, _mm256_set1_epi8(0x70u8 as i8));
        let hi_idx = _mm256_sub_epi8(packed, _mm256_set1_epi8(16));
        let chars = _mm256_or_si256(
            _mm256_shuffle_epi8(lo_table, lo_idx),
            _mm256_shuffle_epi8(hi_table, hi_idx),
        );

        _mm256_storeu_si256(dst.as_mut_ptr().add(dst_offset) as *mut __m256i, chars);

        src_offset += 20;
        dst_offset += 32;
    }

    Ok(dst_offset)
}
