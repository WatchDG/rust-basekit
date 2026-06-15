use crate::common::{assert_untouched, base64::exact_decode_into_no_padding, seed_data};
use basekit::base64::{Base64DecodeConfig, DECODE_TABLE_BASE64, decode64, decode64_into};

fn create_decode_config_no_padding() -> Base64DecodeConfig {
    Base64DecodeConfig::new(DECODE_TABLE_BASE64, None)
}

#[test]
fn test_decode_into_empty() {
    let config = create_decode_config_no_padding();
    let mut dst = vec![0u8; 100];
    let len = decode64_into(&config, &mut dst, b"").unwrap();
    assert_eq!(len, 0);
}

#[test]
fn test_decode_into_consistency_with_decode() {
    let config = create_decode_config_no_padding();
    let data = b"SGVsbG8gV29ybGQh";

    let result = Vec::<u8>::from(decode64(&config, data).unwrap());

    let mut dst = vec![0u8; result.len() + 10];
    let len = decode64_into(&config, &mut dst, data).unwrap();

    assert_eq!(len, result.len());
    assert_eq!(&dst[..len], &result[..]);
}

#[test]
fn test_decode_into_consistency_padded_vs_unpadded() {
    let config_no_pad = create_decode_config_no_padding();
    let config_with_pad = Base64DecodeConfig::new(DECODE_TABLE_BASE64, Some(b'='));

    let unpadded = decode64(&config_no_pad, b"SGVsbG8").unwrap();
    let padded = decode64(&config_with_pad, b"SGVsbG8=").unwrap();

    assert_eq!(Vec::<u8>::from(unpadded), Vec::<u8>::from(padded));
}

#[test]
fn test_decode_into_no_padding_exact_buffer_all_tail_lengths() {
    for size in 1..=9 {
        let data: Vec<u8> = (0..size).map(|i| ((i * 31 + 7) % 256) as u8).collect();
        exact_decode_into_no_padding(&data);
    }
}

#[test]
fn test_decode_into_no_padding_exact_buffer_simd_boundary_sizes() {
    // SIMD encode64 paths process blocks of 12/24/48 input bytes.
    for size in [12, 24, 48] {
        exact_decode_into_no_padding(&seed_data(size));
    }
}

#[test]
fn test_decode_into_no_padding_exact_buffer_large() {
    let data: Vec<u8> = (0..1024).map(|i| (i % 256) as u8).collect();
    exact_decode_into_no_padding(&data);
}

#[test]
fn test_no_write_beyond_returned_length() {
    const MARKER: u8 = 0xCC;

    let config = create_decode_config_no_padding();
    let mut dst = vec![MARKER; 32];
    let len = decode64_into(&config, &mut dst, b"Zm8").unwrap();

    assert_eq!(len, 2);
    assert_eq!(&dst[..len], &[102, 111]);
    assert_untouched(&dst, MARKER, len);
}
