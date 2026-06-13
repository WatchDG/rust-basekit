use crate::common::{
    PATTERNS_2B, PATTERNS_3B,
    base32::{roundtrip_no_padding, roundtrip_no_padding_into},
    seed_data,
};

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
fn test_roundtrip_no_padding_two_bytes() {
    for p in PATTERNS_2B {
        roundtrip_no_padding(p);
        roundtrip_no_padding_into(p);
    }
}

#[test]
fn test_roundtrip_no_padding_three_bytes() {
    for p in PATTERNS_3B {
        roundtrip_no_padding(p);
        roundtrip_no_padding_into(p);
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
    use basekit::base32::{
        ALPHABET_BASE32, Base32DecodeConfig, Base32EncodeConfig, DECODE_TABLE_BASE32, decode,
        encode,
    };

    let enc_config_pad = Base32EncodeConfig::new(ALPHABET_BASE32, Some(b'='));
    let dec_config_pad = Base32DecodeConfig::new(DECODE_TABLE_BASE32, Some(b'='));
    let enc_config_no_pad = Base32EncodeConfig::new(ALPHABET_BASE32, None);
    let dec_config_no_pad = Base32DecodeConfig::new(DECODE_TABLE_BASE32, None);

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
        let data: Vec<u8> = seed_data(size);
        roundtrip_no_padding(&data);
        roundtrip_no_padding_into(&data);
    }
}
