use basekit::base64::{
    ALPHABET_BASE64, Base64DecodeConfig, Base64EncodeConfig, DECODE_TABLE_BASE64, decode,
    decode_into, encode, encode_into,
};

fn create_encode_config() -> Base64EncodeConfig {
    Base64EncodeConfig::new(ALPHABET_BASE64, b'=')
}

fn create_decode_config() -> Base64DecodeConfig {
    Base64DecodeConfig::new(DECODE_TABLE_BASE64, b'=')
}

fn roundtrip(original: &[u8]) {
    let enc_config = create_encode_config();
    let dec_config = create_decode_config();

    let encoded = encode(&enc_config, original);
    let decoded = Vec::<u8>::from(decode(&dec_config, &Vec::<u8>::from(encoded)).unwrap());

    assert_eq!(decoded, original, "Round-trip failed for {:?}", original);
}

fn roundtrip_into(original: &[u8]) {
    let enc_config = create_encode_config();
    let dec_config = create_decode_config();

    let encoded_len = (original.len() / 3 + 1) * 4;
    let mut encoded_dst = vec![0u8; encoded_len];
    let actual_encoded_len = encode_into(&enc_config, &mut encoded_dst, original).unwrap();

    let decoded_len = (actual_encoded_len / 4) * 3;
    let mut decoded_dst = vec![0u8; decoded_len];
    let actual_decoded_len = decode_into(
        &dec_config,
        &mut decoded_dst,
        &encoded_dst[..actual_encoded_len],
    )
    .unwrap();

    assert_eq!(
        &decoded_dst[..actual_decoded_len],
        original,
        "Round-trip failed for {:?}",
        original
    );
}

#[test]
fn test_roundtrip_empty() {
    roundtrip(&[]);
    roundtrip_into(&[]);
}

#[test]
fn test_roundtrip_single_byte() {
    for i in 0u8..=255 {
        roundtrip(&[i]);
        roundtrip_into(&[i]);
    }
}

#[test]
fn test_roundtrip_two_bytes() {
    for i in 0u8..=255 {
        for j in 0u8..=255 {
            roundtrip(&[i, j]);
            roundtrip_into(&[i, j]);
        }
    }
}

#[test]
fn test_roundtrip_three_bytes() {
    let patterns: Vec<Vec<u8>> = vec![
        vec![0, 0, 0],
        vec![0xFF, 0xFF, 0xFF],
        vec![0x00, 0xFF, 0x00],
        vec![0xAA, 0x55, 0xAA],
        vec![1, 2, 3],
        vec![255, 254, 253],
    ];
    for p in patterns {
        roundtrip(&p);
        roundtrip_into(&p);
    }
}

#[test]
fn test_roundtrip_four_bytes() {
    roundtrip(b"foo");
    roundtrip_into(b"foo");

    roundtrip(b"bar");
    roundtrip_into(b"bar");

    roundtrip(b"test");
    roundtrip_into(b"test");
}

#[test]
fn test_roundtrip_strings() {
    let strings = [
        "Hello",
        "Hello!",
        "Hello World",
        "Hello, World!",
        "The quick brown fox jumps over the lazy dog",
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",
        "Spaces and\ttabs\nand\nnewlines",
    ];
    for s in strings {
        roundtrip(s.as_bytes());
        roundtrip_into(s.as_bytes());
    }
}

#[test]
fn test_roundtrip_binary_patterns() {
    let patterns: Vec<Vec<u8>> = vec![
        (0..64).collect(),
        (0..128).collect(),
        (0..192).collect(),
        (0..255).collect(),
        (0..100).step_by(1).collect(),
        (0..100).step_by(2).collect(),
        (0..100).step_by(3).collect(),
        (0..100).step_by(7).collect(),
        (0..100).step_by(13).collect(),
        (0..100).step_by(17).collect(),
    ];
    for p in patterns {
        roundtrip(&p);
        roundtrip_into(&p);
    }
}

#[test]
fn test_roundtrip_all_zeros() {
    let sizes = [1, 2, 3, 4, 5, 10, 50, 100, 255, 256, 1000];
    for size in sizes {
        let data = vec![0u8; size];
        roundtrip(&data);
        roundtrip_into(&data);
    }
}

#[test]
fn test_roundtrip_all_ones() {
    let sizes = [1, 2, 3, 4, 5, 10, 50, 100, 255, 256, 1000];
    for size in sizes {
        let data = vec![0xFFu8; size];
        roundtrip(&data);
        roundtrip_into(&data);
    }
}

#[test]
fn test_roundtrip_alternating_pattern() {
    let sizes = [1, 2, 3, 4, 5, 10, 50, 100, 255, 256];
    for size in sizes {
        let data: Vec<u8> = (0..size)
            .map(|i| if i % 2 == 0 { 0xAA } else { 0x55 })
            .collect();
        roundtrip(&data);
        roundtrip_into(&data);
    }
}

#[test]
fn test_roundtrip_random_like() {
    let seed_data: Vec<u8> = (0..1000).map(|i| ((i * 17 + 42) % 256) as u8).collect();

    for size in [1, 2, 3, 4, 5, 10, 50, 100, 255, 256, 500, 1000] {
        let data: Vec<u8> = seed_data[..size].to_vec();
        roundtrip(&data);
        roundtrip_into(&data);
    }
}

#[test]
fn test_roundtrip_progressive_sizes() {
    for size in 1..=100 {
        let data: Vec<u8> = (0..size).map(|i| (i % 256) as u8).collect();
        roundtrip(&data);
        roundtrip_into(&data);
    }
}

#[test]
fn test_roundtrip_all_sizes_1_to_30() {
    for size in 1..=30 {
        let data: Vec<u8> = (0..size).map(|i| ((i * 7 + 13) % 256) as u8).collect();
        roundtrip(&data);
        roundtrip_into(&data);
    }
}
