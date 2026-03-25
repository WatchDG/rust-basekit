use core::arch::x86_64::*;

use crate::base64::config::Base64DecodeConfig;
use crate::base64::error::Base64Error;

/// Decodes full 4-group blocks (src.len() is a multiple of 16, no padding).
/// Each 16 base64 characters → 12 output bytes.
/// Tail groups and padding are handled by the caller.
#[target_feature(enable = "ssse3")]
#[inline]
#[allow(unsafe_op_in_unsafe_fn)]
pub(crate) unsafe fn decode_full_groups_into_ssse3(
    config: &Base64DecodeConfig,
    dst: &mut [u8],
    src: &[u8],
) -> Result<usize, Base64Error> {
    debug_assert_eq!(src.len() % 16, 0);

    let table_ptr = config.decode_table.as_ptr() as *const __m128i;
    let tbl0 = _mm_loadu_si128(table_ptr);
    let tbl1 = _mm_loadu_si128(table_ptr.add(1));
    let tbl2 = _mm_loadu_si128(table_ptr.add(2));
    let tbl3 = _mm_loadu_si128(table_ptr.add(3));
    let tbl4 = _mm_loadu_si128(table_ptr.add(4));
    let tbl5 = _mm_loadu_si128(table_ptr.add(5));
    let tbl6 = _mm_loadu_si128(table_ptr.add(6));
    let tbl7 = _mm_loadu_si128(table_ptr.add(7));

    // pshufb shuffle: compacts [o2,o1,o0, 0 | o2,o1,o0, 0 | ...] → [o0,o1,o2, o0,o1,o2, ...]
    // indices: [2,1,0, 6,5,4, 10,9,8, 14,13,12, -1,-1,-1,-1]
    // _mm_set_epi8(e15..e0): e0 is byte 0 of result
    let pack_shuf = _mm_set_epi8(-1, -1, -1, -1, 12, 13, 14, 8, 9, 10, 4, 5, 6, 0, 1, 2);

    let mut src_offset = 0usize;
    let mut dst_offset = 0usize;

    while src_offset + 16 <= src.len() {
        let input = _mm_loadu_si128(src.as_ptr().add(src_offset) as *const __m128i);

        // Any byte >= 128 → invalid ASCII, fall back to scalar for this block.
        if _mm_movemask_epi8(input) != 0 {
            let written = scalar_decode_block(
                config,
                &src[src_offset..src_offset + 16],
                src_offset,
                dst,
                dst_offset,
            )?;
            dst_offset += written;
            src_offset += 16;
            continue;
        }

        // Table lookup via 8 × pshufb.
        //
        // upper = bits 6:4 of each input byte (selects one of 8 decode-table blocks).
        // low   = bits 3:0 of each input byte (index within the 16-byte block).
        // sel_k = 0xFF where input's upper nibble == k, else 0x00.
        // r_k   = pshufb(tbl_k, low) & sel_k  (pshufb reads tbl_k[low[i]])
        // decoded = r_0 | r_1 | ... | r_7
        let upper = _mm_and_si128(input, _mm_set1_epi8(0x70u8 as i8));
        let low = _mm_and_si128(input, _mm_set1_epi8(0x0F));

        let sel0 = _mm_cmpeq_epi8(upper, _mm_setzero_si128());
        let sel1 = _mm_cmpeq_epi8(upper, _mm_set1_epi8(0x10u8 as i8));
        let sel2 = _mm_cmpeq_epi8(upper, _mm_set1_epi8(0x20u8 as i8));
        let sel3 = _mm_cmpeq_epi8(upper, _mm_set1_epi8(0x30u8 as i8));
        let sel4 = _mm_cmpeq_epi8(upper, _mm_set1_epi8(0x40u8 as i8));
        let sel5 = _mm_cmpeq_epi8(upper, _mm_set1_epi8(0x50u8 as i8));
        let sel6 = _mm_cmpeq_epi8(upper, _mm_set1_epi8(0x60u8 as i8));
        let sel7 = _mm_cmpeq_epi8(upper, _mm_set1_epi8(0x70u8 as i8));

        let r0 = _mm_and_si128(_mm_shuffle_epi8(tbl0, low), sel0);
        let r1 = _mm_and_si128(_mm_shuffle_epi8(tbl1, low), sel1);
        let r2 = _mm_and_si128(_mm_shuffle_epi8(tbl2, low), sel2);
        let r3 = _mm_and_si128(_mm_shuffle_epi8(tbl3, low), sel3);
        let r4 = _mm_and_si128(_mm_shuffle_epi8(tbl4, low), sel4);
        let r5 = _mm_and_si128(_mm_shuffle_epi8(tbl5, low), sel5);
        let r6 = _mm_and_si128(_mm_shuffle_epi8(tbl6, low), sel6);
        let r7 = _mm_and_si128(_mm_shuffle_epi8(tbl7, low), sel7);

        let decoded = _mm_or_si128(
            _mm_or_si128(_mm_or_si128(r0, r1), _mm_or_si128(r2, r3)),
            _mm_or_si128(_mm_or_si128(r4, r5), _mm_or_si128(r6, r7)),
        );

        // Any 0xFF in decoded means an invalid character in the table.
        if _mm_movemask_epi8(_mm_cmpeq_epi8(decoded, _mm_set1_epi8(-1))) != 0 {
            let written = scalar_decode_block(
                config,
                &src[src_offset..src_offset + 16],
                src_offset,
                dst,
                dst_offset,
            )?;
            dst_offset += written;
            src_offset += 16;
            continue;
        }

        // Pack 16 × 6-bit decoded values into 12 output bytes (Muła/Lemire algorithm).
        //
        // Step A — maddubs: merge adjacent pairs (a, b) → a*64 + b (12-bit in 16-bit field).
        //   decoded[i] ∈ [0..63], so a*64+b ≤ 4095, no i16 saturation.
        let t0 = _mm_maddubs_epi16(decoded, _mm_set1_epi32(0x01400140u32 as i32));

        // Step B — madd: merge adjacent 12-bit pairs (c, d) → c<<12 | d (24-bit in 32-bit field).
        //   In LE memory the 32-bit word is: [o2, o1, o0, 0x00].
        let t1 = _mm_madd_epi16(t0, _mm_set1_epi32(0x00011000u32 as i32));

        // Step C — pshufb: reorder bytes to produce [o0_a,o1_a,o2_a, o0_b,o1_b,o2_b, ...].
        let packed = _mm_shuffle_epi8(t1, pack_shuf);

        // Write exactly 12 bytes: 8 via storel_epi64, then 4 via scalar.
        let out_ptr = dst.as_mut_ptr().add(dst_offset);
        _mm_storel_epi64(out_ptr as *mut __m128i, packed);
        let shifted = _mm_srli_si128(packed, 8);
        let word = _mm_cvtsi128_si32(shifted);
        core::ptr::write_unaligned(out_ptr.add(8) as *mut i32, word);

        src_offset += 16;
        dst_offset += 12;
    }

    Ok(dst_offset)
}

/// Scalar fallback for a single 4-group block (16 input bytes, 12 output bytes).
/// `block_src_start` is the byte offset of this block within the original source slice,
/// used for accurate error position reporting.
#[inline(never)]
#[allow(unsafe_op_in_unsafe_fn)]
unsafe fn scalar_decode_block(
    config: &Base64DecodeConfig,
    block: &[u8],
    block_src_start: usize,
    dst: &mut [u8],
    dst_offset: usize,
) -> Result<usize, Base64Error> {
    debug_assert_eq!(block.len(), 16);
    let decode_table = config.decode_table;
    let mut written = 0usize;

    for group_rel in 0..4usize {
        let chunk_start = block_src_start + group_rel * 4;
        let chunk = &block[group_rel * 4..group_rel * 4 + 4];

        let b0 = chunk[0];
        let b1 = chunk[1];
        let b2 = chunk[2];
        let b3 = chunk[3];

        if b0 == config.padding
            || b1 == config.padding
            || b2 == config.padding
            || b3 == config.padding
        {
            return Err(Base64Error::InvalidPadding);
        }

        if (b0 | b1 | b2 | b3) & 0x80 != 0 {
            let pos = if b0 & 0x80 != 0 {
                0
            } else if b1 & 0x80 != 0 {
                1
            } else if b2 & 0x80 != 0 {
                2
            } else {
                3
            };
            return Err(Base64Error::InvalidCharacter(chunk[pos], chunk_start + pos));
        }

        let i0 = decode_table[b0 as usize];
        let i1 = decode_table[b1 as usize];
        let i2 = decode_table[b2 as usize];
        let i3 = decode_table[b3 as usize];

        if i0 < 0 || i1 < 0 || i2 < 0 || i3 < 0 {
            let pos = if i0 < 0 {
                0
            } else if i1 < 0 {
                1
            } else if i2 < 0 {
                2
            } else {
                3
            };
            return Err(Base64Error::InvalidCharacter(chunk[pos], chunk_start + pos));
        }

        let triple = ((i0 as u32) << 18) | ((i1 as u32) << 12) | ((i2 as u32) << 6) | (i3 as u32);

        let ptr = dst.as_mut_ptr().add(dst_offset + written);
        ptr.write((triple >> 16) as u8);
        ptr.add(1).write((triple >> 8) as u8);
        ptr.add(2).write(triple as u8);

        written += 3;
    }

    Ok(written)
}
