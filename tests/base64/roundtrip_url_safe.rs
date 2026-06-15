use crate::common::{PATTERNS_2B, PATTERNS_3B, STRINGS, assert_untouched, seed_data};
use basekit::base64::{
    ALPHABET_BASE64, ALPHABET_BASE64_URL, Base64DecodeConfig, Base64EncodeConfig,
    DECODE_TABLE_BASE64, DECODE_TABLE_BASE64_URL, decode64, decode64_into, encode64, encode64_into,
};

fn enc_config() -> Base64EncodeConfig {
    Base64EncodeConfig::new(ALPHABET_BASE64_URL, Some(b'='))
}

fn dec_config() -> Base64DecodeConfig {
    Base64DecodeConfig::new(DECODE_TABLE_BASE64_URL, Some(b'='))
}

fn enc_config_no_padding() -> Base64EncodeConfig {
    Base64EncodeConfig::new(ALPHABET_BASE64_URL, None)
}

fn dec_config_no_padding() -> Base64DecodeConfig {
    Base64DecodeConfig::new(DECODE_TABLE_BASE64_URL, None)
}

fn encoded_len_padded(len: usize) -> usize {
    (len + 2) / 3 * 4
}

fn roundtrip(original: &[u8]) {
    let encoded = Vec::<u8>::from(encode64(&enc_config(), original));
    let decoded = Vec::<u8>::from(decode64(&dec_config(), &encoded).unwrap());
    assert_eq!(
        decoded, original,
        "URL-safe round-trip failed for {:?}",
        original
    );
}

fn roundtrip_into(original: &[u8]) {
    let enc_len = encoded_len_padded(original.len());
    let mut enc_dst = vec![0u8; enc_len];
    let actual_enc_len = encode64_into(&enc_config(), &mut enc_dst, original).unwrap();
    assert_eq!(actual_enc_len, enc_len);

    let mut dec_dst = vec![0u8; original.len()];
    let actual_dec_len =
        decode64_into(&dec_config(), &mut dec_dst, &enc_dst[..actual_enc_len]).unwrap();
    assert_eq!(
        &dec_dst[..actual_dec_len],
        original,
        "URL-safe round-trip (into) failed for {:?}",
        original
    );
}

fn roundtrip_no_padding(original: &[u8]) {
    let encoded = Vec::<u8>::from(encode64(&enc_config_no_padding(), original));
    let decoded = Vec::<u8>::from(decode64(&dec_config_no_padding(), &encoded).unwrap());
    assert_eq!(
        decoded, original,
        "URL-safe no-padding round-trip failed for {:?}",
        original
    );
}

#[test]
fn test_empty() {
    roundtrip(&[]);
    roundtrip_into(&[]);
    roundtrip_no_padding(&[]);
}

#[test]
fn test_single_byte() {
    for i in 0u8..=255 {
        roundtrip(&[i]);
        roundtrip_into(&[i]);
    }
}

#[test]
fn test_two_bytes() {
    for p in PATTERNS_2B {
        roundtrip(p);
        roundtrip_into(p);
        roundtrip_no_padding(p);
    }
}

#[test]
fn test_three_bytes() {
    for p in PATTERNS_3B {
        roundtrip(p);
        roundtrip_into(p);
        roundtrip_no_padding(p);
    }
}

#[test]
fn test_strings() {
    for s in STRINGS {
        roundtrip(s.as_bytes());
        roundtrip_into(s.as_bytes());
        roundtrip_no_padding(s.as_bytes());
    }
}

#[test]
fn test_binary_patterns() {
    let patterns: Vec<Vec<u8>> = vec![
        (0..64).collect(),
        (0..128).collect(),
        (0..192).collect(),
        (0..255).collect(),
        seed_data(100),
        seed_data(256),
    ];
    for p in patterns {
        roundtrip(&p);
        roundtrip_into(&p);
    }
}

#[test]
fn test_url_safe_differs_from_standard() {
    // [0x00, 0x00, 0xFF] encodes to "AAD/" in standard Base64 and "AAD_" in URL-safe.
    let data = [0x00, 0x00, 0xFF];

    let standard = encode64(&Base64EncodeConfig::new(ALPHABET_BASE64, Some(b'=')), &data);
    let url_safe = encode64(&enc_config(), &data);

    assert_eq!(Vec::<u8>::from(standard), b"AAD/");
    assert_eq!(Vec::<u8>::from(url_safe), b"AAD_");
}

#[test]
fn test_standard_decode_rejects_url_safe_characters() {
    let standard_dec = Base64DecodeConfig::new(DECODE_TABLE_BASE64, Some(b'='));
    let result = decode64(&standard_dec, b"AAD_");

    assert!(
        matches!(
            result,
            Err(basekit::base64::Base64Error::InvalidCharacter(b'_', 3))
        ),
        "standard decoder should reject URL-safe '_' character"
    );
}

#[test]
fn test_no_write_beyond_returned_length() {
    const MARKER: u8 = 0xCC;

    let mut dst = vec![MARKER; 32];
    let len = encode64_into(&enc_config(), &mut dst, b"f").unwrap();

    assert_eq!(len, 4);
    assert_eq!(&dst[..len], b"Zg==");
    assert_untouched(&dst, MARKER, len);
}
