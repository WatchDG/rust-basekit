use crate::common::{assert_untouched, base32::exact_decode_into_no_padding, seed_data};

#[test]
fn test_decode_into_no_padding_empty() {
    exact_decode_into_no_padding(b"");
}

#[test]
fn test_decode_into_no_padding_all_tail_lengths() {
    // Input lengths 1..=9 cover all possible unpadded tail char counts.
    for size in 1..=9 {
        let data: Vec<u8> = (0..size).map(|i| ((i * 31 + 7) % 256) as u8).collect();
        exact_decode_into_no_padding(&data);
    }
}

#[test]
fn test_decode_into_no_padding_simd_boundary_sizes() {
    // SIMD encode paths process blocks of 10/20/40 input bytes.
    for size in [10, 20, 40] {
        exact_decode_into_no_padding(&seed_data(size));
    }
}

#[test]
fn test_decode_into_no_padding_large() {
    let data: Vec<u8> = (0..1024).map(|i| (i % 256) as u8).collect();
    exact_decode_into_no_padding(&data);
}

#[test]
fn test_no_write_beyond_returned_length() {
    const MARKER: u8 = 0xCC;

    let mut dst = vec![MARKER; 32];
    let len = {
        use basekit::base32::{Base32DecodeConfig, DECODE_TABLE_BASE32, decode_into};
        let config = Base32DecodeConfig::new(DECODE_TABLE_BASE32, None);
        decode_into(&config, &mut dst, b"MY").unwrap()
    };

    assert_eq!(len, 1);
    assert_eq!(dst[0], 102);
    assert_untouched(&dst, MARKER, len);
}
