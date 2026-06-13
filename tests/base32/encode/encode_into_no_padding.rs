use crate::common::{assert_untouched, base32::exact_encode_into_no_padding, seed_data};

#[test]
fn test_encode_into_no_padding_empty() {
    exact_encode_into_no_padding(b"");
}

#[test]
fn test_encode_into_no_padding_all_tail_lengths() {
    // Input lengths 1..=9 cover all possible unpadded tail output lengths.
    for size in 1..=9 {
        let data: Vec<u8> = (0..size).map(|i| ((i * 31 + 7) % 256) as u8).collect();
        exact_encode_into_no_padding(&data);
    }
}

#[test]
fn test_encode_into_no_padding_simd_boundary_sizes() {
    // SIMD encode paths process blocks of 10/20/40 input bytes.
    for size in [10, 20, 40] {
        exact_encode_into_no_padding(&seed_data(size));
    }
}

#[test]
fn test_encode_into_no_padding_large() {
    let data: Vec<u8> = (0..1024).map(|i| (i % 256) as u8).collect();
    exact_encode_into_no_padding(&data);
}

#[test]
fn test_no_write_beyond_returned_length() {
    const MARKER: u8 = 0xCC;

    let mut dst = vec![MARKER; 32];
    let len = {
        use basekit::base32::{ALPHABET_BASE32, Base32EncodeConfig, encode_into};
        let config = Base32EncodeConfig::new(ALPHABET_BASE32, None);
        encode_into(&config, &mut dst, b"f").unwrap()
    };

    assert_eq!(len, 2);
    assert_eq!(&dst[..len], b"MY");
    assert_untouched(&dst, MARKER, len);
}
