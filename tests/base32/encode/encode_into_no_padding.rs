use basekit::base32::{ALPHABET_BASE32, Base32EncodeConfig, encode, encode_into};

fn create_config() -> Base32EncodeConfig {
    Base32EncodeConfig::new(ALPHABET_BASE32, None)
}

fn exact_encode_into(data: &[u8]) {
    let config = create_config();
    let expected = Vec::<u8>::from(encode(&config, data));

    let mut dst = vec![0u8; expected.len()];
    let len = encode_into(&config, &mut dst, data).unwrap();

    assert_eq!(len, expected.len());
    assert_eq!(&dst[..len], &expected[..]);
}

#[test]
fn test_encode_into_no_padding_empty() {
    exact_encode_into(b"");
}

#[test]
fn test_encode_into_no_padding_all_tail_lengths() {
    // Input lengths 1..=9 cover all possible unpadded tail output lengths.
    for size in 1..=9 {
        let data: Vec<u8> = (0..size).map(|i| ((i * 31 + 7) % 256) as u8).collect();
        exact_encode_into(&data);
    }
}

#[test]
fn test_encode_into_no_padding_simd_boundary_sizes() {
    // SIMD encode paths process blocks of 10/20/40 input bytes.
    for size in [10, 20, 40] {
        let data: Vec<u8> = (0..size).map(|i| ((i * 17 + 42) % 256) as u8).collect();
        exact_encode_into(&data);
    }
}

#[test]
fn test_encode_into_no_padding_large() {
    let data: Vec<u8> = (0..1024).map(|i| (i % 256) as u8).collect();
    exact_encode_into(&data);
}
