use basekit::base32::{
    ALPHABET_BASE32, Base32DecodeConfig, Base32EncodeConfig, DECODE_TABLE_BASE32, decode,
    decode_into, encode, encode_into,
};

fn create_encode_config() -> Base32EncodeConfig {
    Base32EncodeConfig::new(ALPHABET_BASE32, None)
}

fn create_decode_config() -> Base32DecodeConfig {
    Base32DecodeConfig::new(DECODE_TABLE_BASE32, None)
}

fn roundtrip_no_padding(original: &[u8]) {
    let enc_config = create_encode_config();
    let dec_config = create_decode_config();

    let encoded = encode(&enc_config, original);
    let decoded = Vec::<u8>::from(decode(&dec_config, &Vec::<u8>::from(encoded)).unwrap());

    assert_eq!(
        decoded, original,
        "Round-trip no-padding failed for {:?}",
        original
    );
}

fn roundtrip_no_padding_into(original: &[u8]) {
    let enc_config = create_encode_config();
    let dec_config = create_decode_config();

    let max_encoded_len = (original.len() / 5 + 1) * 8;
    let mut encoded_dst = vec![0u8; max_encoded_len];
    let actual_encoded_len = encode_into(&enc_config, &mut encoded_dst, original).unwrap();

    let expected =
        Vec::<u8>::from(decode(&dec_config, &encoded_dst[..actual_encoded_len]).unwrap());
    let mut decoded_dst = vec![0u8; expected.len()];
    let actual_decoded_len = decode_into(
        &dec_config,
        &mut decoded_dst,
        &encoded_dst[..actual_encoded_len],
    )
    .unwrap();

    assert_eq!(
        &decoded_dst[..actual_decoded_len],
        original,
        "Round-trip no-padding (into) failed for {:?}",
        original
    );
}

#[test]
fn test_roundtrip_no_padding_empty() {
    roundtrip_no_padding(&[]);
    roundtrip_no_padding_into(&[]);
}

#[test]
fn test_roundtrip_no_padding_single_byte() {
    for i in 0u8..=255 {
        roundtrip_no_padding(&[i]);
        roundtrip_no_padding_into(&[i]);
    }
}

#[test]
fn test_roundtrip_no_padding_strings() {
    let strings = [
        "Hello",
        "Hello World",
        "The quick brown fox jumps over the lazy dog",
        "Spaces and\ttabs\nand\nnewlines",
    ];
    for s in strings {
        roundtrip_no_padding(s.as_bytes());
        roundtrip_no_padding_into(s.as_bytes());
    }
}

#[test]
fn test_roundtrip_no_padding_consistency_with_padded() {
    let enc_config_pad = Base32EncodeConfig::new(ALPHABET_BASE32, Some(b'='));
    let dec_config_pad = Base32DecodeConfig::new(DECODE_TABLE_BASE32, Some(b'='));
    let enc_config_no_pad = create_encode_config();
    let dec_config_no_pad = create_decode_config();

    let data = b"Hello, World! The quick brown fox jumps over the lazy dog.";

    let encoded_pad = encode(&enc_config_pad, data);
    let encoded_no_pad = encode(&enc_config_no_pad, data);

    let decoded_pad =
        Vec::<u8>::from(decode(&dec_config_pad, &Vec::<u8>::from(encoded_pad)).unwrap());
    let decoded_no_pad =
        Vec::<u8>::from(decode(&dec_config_no_pad, &Vec::<u8>::from(encoded_no_pad)).unwrap());

    assert_eq!(decoded_pad, decoded_no_pad);
    assert_eq!(decoded_pad, data);
}

#[test]
fn test_roundtrip_no_padding_binary_patterns() {
    let patterns: Vec<Vec<u8>> = vec![
        (0..32).collect(),
        (0..64).collect(),
        (0..128).collect(),
        (0..255).collect(),
    ];
    for p in patterns {
        roundtrip_no_padding(&p);
        roundtrip_no_padding_into(&p);
    }
}

#[test]
fn test_roundtrip_no_padding_all_zeros() {
    for size in [1, 2, 3, 4, 5, 10, 50, 100] {
        let data = vec![0u8; size];
        roundtrip_no_padding(&data);
        roundtrip_no_padding_into(&data);
    }
}

#[test]
fn test_roundtrip_no_padding_all_ones() {
    for size in [1, 2, 3, 4, 5, 10, 50, 100] {
        let data = vec![0xFFu8; size];
        roundtrip_no_padding(&data);
        roundtrip_no_padding_into(&data);
    }
}

#[test]
fn test_roundtrip_no_padding_progressive_sizes() {
    for size in 1..=100 {
        let data: Vec<u8> = (0..size).map(|i| (i % 256) as u8).collect();
        roundtrip_no_padding(&data);
        roundtrip_no_padding_into(&data);
    }
}

#[test]
fn test_roundtrip_no_padding_simd_boundary_sizes() {
    // SIMD encode paths process blocks of 10/20/40 input bytes.
    for size in [10, 20, 40] {
        let data: Vec<u8> = (0..size).map(|i| ((i * 17 + 42) % 256) as u8).collect();
        roundtrip_no_padding(&data);
        roundtrip_no_padding_into(&data);
    }
}
