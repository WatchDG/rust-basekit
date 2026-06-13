use basekit::base32::{
    ALPHABET_BASE32, Base32DecodeConfig, Base32EncodeConfig, DECODE_TABLE_BASE32, decode,
    decode_into, encode,
};

fn create_decode_config() -> Base32DecodeConfig {
    Base32DecodeConfig::new(DECODE_TABLE_BASE32, None)
}

fn create_encode_config() -> Base32EncodeConfig {
    Base32EncodeConfig::new(ALPHABET_BASE32, None)
}

fn exact_decode_into(data: &[u8]) {
    let enc_config = create_encode_config();
    let dec_config = create_decode_config();

    let encoded = Vec::<u8>::from(encode(&enc_config, data));
    let expected = Vec::<u8>::from(decode(&dec_config, &encoded).unwrap());

    let mut dst = vec![0u8; expected.len()];
    let len = decode_into(&dec_config, &mut dst, &encoded).unwrap();

    assert_eq!(len, expected.len());
    assert_eq!(&dst[..len], &expected[..]);
}

#[test]
fn test_decode_into_no_padding_empty() {
    exact_decode_into(b"");
}

#[test]
fn test_decode_into_no_padding_all_tail_lengths() {
    // Input lengths 1..=9 cover all possible unpadded tail char counts.
    for size in 1..=9 {
        let data: Vec<u8> = (0..size).map(|i| ((i * 31 + 7) % 256) as u8).collect();
        exact_decode_into(&data);
    }
}

#[test]
fn test_decode_into_no_padding_simd_boundary_sizes() {
    // SIMD encode paths process blocks of 10/20/40 input bytes.
    for size in [10, 20, 40] {
        let data: Vec<u8> = (0..size).map(|i| ((i * 17 + 42) % 256) as u8).collect();
        exact_decode_into(&data);
    }
}

#[test]
fn test_decode_into_no_padding_large() {
    let data: Vec<u8> = (0..1024).map(|i| (i % 256) as u8).collect();
    exact_decode_into(&data);
}
