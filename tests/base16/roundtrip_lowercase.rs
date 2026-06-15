use crate::common::{
    PATTERNS_2B, PATTERNS_3B, STRINGS,
    base16::{roundtrip, roundtrip_into},
    seed_data,
};
use basekit::base16::{
    ALPHABET_BASE16_LOWERCASE, Base16DecodeConfig, Base16EncodeConfig,
    DECODE_TABLE_BASE16_LOWERCASE,
};

fn enc_config() -> Base16EncodeConfig {
    Base16EncodeConfig::new(ALPHABET_BASE16_LOWERCASE)
}

fn dec_config() -> Base16DecodeConfig {
    Base16DecodeConfig::new(DECODE_TABLE_BASE16_LOWERCASE)
}

fn lowercase_roundtrip(original: &[u8]) {
    let encoded = Vec::<u8>::from(basekit::base16::encode16(&enc_config(), original));
    let decoded = Vec::<u8>::from(basekit::base16::decode16(&dec_config(), &encoded).unwrap());
    assert_eq!(
        decoded, original,
        "Lowercase round-trip failed for {:?}",
        original
    );
}

fn lowercase_roundtrip_into(original: &[u8]) {
    let enc_len = original.len() * 2;
    let mut enc_dst = vec![0u8; enc_len];
    let actual_enc_len =
        basekit::base16::encode16_into(&enc_config(), &mut enc_dst, original).unwrap();
    assert_eq!(actual_enc_len, enc_len);

    let mut dec_dst = vec![0u8; original.len()];
    let actual_dec_len =
        basekit::base16::decode16_into(&dec_config(), &mut dec_dst, &enc_dst).unwrap();
    assert_eq!(
        &dec_dst[..actual_dec_len],
        original,
        "Lowercase round-trip (into) failed for {:?}",
        original
    );
}

#[test]
fn test_roundtrip_lowercase_empty() {
    lowercase_roundtrip(&[]);
    lowercase_roundtrip_into(&[]);
}

#[test]
fn test_roundtrip_lowercase_single_byte() {
    for i in 0u8..=255 {
        lowercase_roundtrip(&[i]);
        lowercase_roundtrip_into(&[i]);
    }
}

#[test]
fn test_roundtrip_lowercase_two_bytes() {
    for p in PATTERNS_2B {
        lowercase_roundtrip(p);
        lowercase_roundtrip_into(p);
    }
}

#[test]
fn test_roundtrip_lowercase_three_bytes() {
    for p in PATTERNS_3B {
        lowercase_roundtrip(p);
        lowercase_roundtrip_into(p);
    }
}

#[test]
fn test_roundtrip_lowercase_strings() {
    for s in STRINGS {
        lowercase_roundtrip(s.as_bytes());
        lowercase_roundtrip_into(s.as_bytes());
    }
}

#[test]
fn test_roundtrip_lowercase_binary_patterns() {
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
        lowercase_roundtrip(&p);
        lowercase_roundtrip_into(&p);
    }
}

#[test]
fn test_roundtrip_lowercase_all_zeros() {
    for size in [1, 2, 3, 4, 5, 10, 50, 100, 255, 256, 1000] {
        let data = vec![0u8; size];
        lowercase_roundtrip(&data);
        lowercase_roundtrip_into(&data);
    }
}

#[test]
fn test_roundtrip_lowercase_all_ones() {
    for size in [1, 2, 3, 4, 5, 10, 50, 100, 255, 256, 1000] {
        let data = vec![0xFFu8; size];
        lowercase_roundtrip(&data);
        lowercase_roundtrip_into(&data);
    }
}

#[test]
fn test_roundtrip_lowercase_alternating_pattern() {
    for size in [1, 2, 3, 4, 5, 10, 50, 100, 255, 256] {
        let data: Vec<u8> = (0..size)
            .map(|i| if i % 2 == 0 { 0xAA } else { 0x55 })
            .collect();
        lowercase_roundtrip(&data);
        lowercase_roundtrip_into(&data);
    }
}

#[test]
fn test_roundtrip_lowercase_random_like() {
    let seed = seed_data(1000);

    for size in [1, 2, 3, 4, 5, 10, 50, 100, 255, 256, 500, 1000] {
        let data: Vec<u8> = seed[..size].to_vec();
        lowercase_roundtrip(&data);
        lowercase_roundtrip_into(&data);
    }
}

#[test]
fn test_roundtrip_lowercase_progressive_sizes() {
    for size in 1..=100 {
        let data: Vec<u8> = (0..size).map(|i| (i % 256) as u8).collect();
        lowercase_roundtrip(&data);
        lowercase_roundtrip_into(&data);
    }
}

#[test]
fn test_roundtrip_lowercase_all_sizes_1_to_50() {
    for size in 1..=50 {
        let data: Vec<u8> = (0..size).map(|i| ((i * 7 + 13) % 256) as u8).collect();
        lowercase_roundtrip(&data);
        lowercase_roundtrip_into(&data);
    }
}

#[test]
fn test_lowercase_differs_from_uppercase() {
    let data = [0xAB, 0xCD, 0xEF];

    let uppercase = basekit::base16::encode16(
        &Base16EncodeConfig::new(basekit::base16::ALPHABET_BASE16_UPPERCASE),
        &data,
    );
    let lowercase = basekit::base16::encode16(&enc_config(), &data);

    assert_eq!(Vec::<u8>::from(uppercase), b"ABCDEF");
    assert_eq!(Vec::<u8>::from(lowercase), b"abcdef");
}

#[test]
fn test_uppercase_decode_rejects_lowercase_characters() {
    let uppercase_dec = Base16DecodeConfig::new(basekit::base16::DECODE_TABLE_BASE16_UPPERCASE);
    let result = basekit::base16::decode16(&uppercase_dec, b"abcd");

    assert!(
        matches!(
            result,
            Err(basekit::base16::Base16Error::InvalidCharacter(b'a', 0))
        ),
        "uppercase decoder should reject lowercase characters"
    );
}
