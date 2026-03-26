use core::arch::x86_64::*;

use super::super::decode_full_group_into::decode_full_group_into;
use crate::base64::config::Base64DecodeConfig;
use crate::base64::error::Base64Error;

/// Decodes full 16-group blocks (src.len() is a multiple of 64, no padding).
/// Each 64 base64 characters → 48 output bytes.
/// Tail groups and padding are handled by the caller.
#[target_feature(enable = "avx512f,avx512bw")]
#[inline]
#[allow(unsafe_op_in_unsafe_fn)]
pub(crate) unsafe fn avx512_decode_full_groups_into(
    config: &Base64DecodeConfig,
    dst: &mut [u8],
    src: &[u8],
) -> Result<usize, Base64Error> {
    debug_assert_eq!(src.len() % 64, 0);

    let table_ptr = config.decode_table.as_ptr() as *const __m128i;
    let tbl0 = _mm512_broadcast_i32x4(_mm_loadu_si128(table_ptr));
    let tbl1 = _mm512_broadcast_i32x4(_mm_loadu_si128(table_ptr.add(1)));
    let tbl2 = _mm512_broadcast_i32x4(_mm_loadu_si128(table_ptr.add(2)));
    let tbl3 = _mm512_broadcast_i32x4(_mm_loadu_si128(table_ptr.add(3)));
    let tbl4 = _mm512_broadcast_i32x4(_mm_loadu_si128(table_ptr.add(4)));
    let tbl5 = _mm512_broadcast_i32x4(_mm_loadu_si128(table_ptr.add(5)));
    let tbl6 = _mm512_broadcast_i32x4(_mm_loadu_si128(table_ptr.add(6)));
    let tbl7 = _mm512_broadcast_i32x4(_mm_loadu_si128(table_ptr.add(7)));

    let pack_shuf = _mm512_broadcast_i32x4(_mm_set_epi8(
        -1, -1, -1, -1, 12, 13, 14, 8, 9, 10, 4, 5, 6, 0, 1, 2,
    ));

    // Compact 12 valid bytes from each 128-bit lane into contiguous 48 bytes.
    // Lane 0: dwords 0,1,2 | Lane 1: dwords 4,5,6 | Lane 2: dwords 8,9,10 | Lane 3: dwords 12,13,14
    let perm_idx = _mm512_set_epi32(0, 0, 0, 0, 14, 13, 12, 10, 9, 8, 6, 5, 4, 2, 1, 0);

    let mut src_offset = 0usize;
    let mut dst_offset = 0usize;

    while src_offset + 64 <= src.len() {
        let input = _mm512_loadu_si512(src.as_ptr().add(src_offset) as *const __m512i);

        if _mm512_movepi8_mask(input) != 0 {
            let mut written = 0usize;
            for group_offset in (0..64).step_by(4) {
                written += decode_full_group_into(
                    config,
                    &mut dst[dst_offset + written..],
                    &src[src_offset + group_offset..src_offset + group_offset + 4],
                    src_offset + group_offset,
                )?;
            }
            dst_offset += written;
            src_offset += 64;
            continue;
        }

        let upper = _mm512_and_si512(input, _mm512_set1_epi8(0x70u8 as i8));
        let low = _mm512_and_si512(input, _mm512_set1_epi8(0x0F));

        let mask0 = _mm512_cmpeq_epi8_mask(upper, _mm512_setzero_si512());
        let mask1 = _mm512_cmpeq_epi8_mask(upper, _mm512_set1_epi8(0x10u8 as i8));
        let mask2 = _mm512_cmpeq_epi8_mask(upper, _mm512_set1_epi8(0x20u8 as i8));
        let mask3 = _mm512_cmpeq_epi8_mask(upper, _mm512_set1_epi8(0x30u8 as i8));
        let mask4 = _mm512_cmpeq_epi8_mask(upper, _mm512_set1_epi8(0x40u8 as i8));
        let mask5 = _mm512_cmpeq_epi8_mask(upper, _mm512_set1_epi8(0x50u8 as i8));
        let mask6 = _mm512_cmpeq_epi8_mask(upper, _mm512_set1_epi8(0x60u8 as i8));
        let mask7 = _mm512_cmpeq_epi8_mask(upper, _mm512_set1_epi8(0x70u8 as i8));

        let r0 = _mm512_maskz_shuffle_epi8(mask0, tbl0, low);
        let r1 = _mm512_maskz_shuffle_epi8(mask1, tbl1, low);
        let r2 = _mm512_maskz_shuffle_epi8(mask2, tbl2, low);
        let r3 = _mm512_maskz_shuffle_epi8(mask3, tbl3, low);
        let r4 = _mm512_maskz_shuffle_epi8(mask4, tbl4, low);
        let r5 = _mm512_maskz_shuffle_epi8(mask5, tbl5, low);
        let r6 = _mm512_maskz_shuffle_epi8(mask6, tbl6, low);
        let r7 = _mm512_maskz_shuffle_epi8(mask7, tbl7, low);

        let decoded = _mm512_or_si512(
            _mm512_or_si512(_mm512_or_si512(r0, r1), _mm512_or_si512(r2, r3)),
            _mm512_or_si512(_mm512_or_si512(r4, r5), _mm512_or_si512(r6, r7)),
        );

        if _mm512_cmpeq_epi8_mask(decoded, _mm512_set1_epi8(-1)) != 0 {
            let mut written = 0usize;
            for group_offset in (0..64).step_by(4) {
                written += decode_full_group_into(
                    config,
                    &mut dst[dst_offset + written..],
                    &src[src_offset + group_offset..src_offset + group_offset + 4],
                    src_offset + group_offset,
                )?;
            }
            dst_offset += written;
            src_offset += 64;
            continue;
        }

        // Pack 64 x 6-bit decoded values into 48 output bytes (Muła/Lemire algorithm, per lane).
        let t0 = _mm512_maddubs_epi16(decoded, _mm512_set1_epi32(0x01400140u32 as i32));
        let t1 = _mm512_madd_epi16(t0, _mm512_set1_epi32(0x00011000u32 as i32));
        let packed = _mm512_shuffle_epi8(t1, pack_shuf);
        let result = _mm512_permutexvar_epi32(perm_idx, packed);

        let out_ptr = dst.as_mut_ptr().add(dst_offset);
        _mm256_storeu_si256(out_ptr as *mut __m256i, _mm512_castsi512_si256(result));
        _mm_storeu_si128(
            out_ptr.add(32) as *mut __m128i,
            _mm512_extracti32x4_epi32::<2>(result),
        );

        src_offset += 64;
        dst_offset += 48;
    }

    Ok(dst_offset)
}
