use basekit::base64::{Base64DecodeConfig, DECODE_TABLE_BASE64, decode};

fn create_config_no_padding() -> Base64DecodeConfig {
    Base64DecodeConfig::new(DECODE_TABLE_BASE64, None)
}

#[test]
fn test_decode_empty() {
    let config = create_config_no_padding();
    let result = decode(&config, b"");
    assert_eq!(Vec::<u8>::from(result.unwrap()), b"");
}

#[test]
fn test_decode_single_byte_no_padding() {
    let config = create_config_no_padding();
    let result = decode(&config, b"Zg");
    assert_eq!(Vec::<u8>::from(result.unwrap()), &[102]);
}

#[test]
fn test_decode_two_bytes_no_padding() {
    let config = create_config_no_padding();
    let result = decode(&config, b"Zm8");
    assert_eq!(Vec::<u8>::from(result.unwrap()), &[102, 111]);
}

#[test]
fn test_decode_three_bytes_no_padding() {
    let config = create_config_no_padding();
    let result = decode(&config, b"Zm9v");
    assert_eq!(Vec::<u8>::from(result.unwrap()), &[102, 111, 111]);
}

#[test]
fn test_decode_four_bytes_no_padding() {
    let config = create_config_no_padding();
    let result = decode(&config, b"Zm9vYg");
    assert_eq!(Vec::<u8>::from(result.unwrap()), &[102, 111, 111, 98]);
}

#[test]
fn test_decode_five_bytes_no_padding() {
    let config = create_config_no_padding();
    let result = decode(&config, b"Zm9vYmE");
    assert_eq!(Vec::<u8>::from(result.unwrap()), &[102, 111, 111, 98, 97]);
}

#[test]
fn test_decode_six_bytes_no_padding() {
    let config = create_config_no_padding();
    let result = decode(&config, b"Zm9vYmFy");
    assert_eq!(
        Vec::<u8>::from(result.unwrap()),
        &[102, 111, 111, 98, 97, 114]
    );
}

#[test]
fn test_decode_hello_no_padding() {
    let config = create_config_no_padding();
    let result = decode(&config, b"SGVsbG8");
    assert_eq!(Vec::<u8>::from(result.unwrap()), b"Hello");
}

#[test]
fn test_decode_all_zeros_no_padding() {
    let config = create_config_no_padding();
    let result = decode(&config, b"AAAA");
    assert_eq!(Vec::<u8>::from(result.unwrap()), &[0, 0, 0]);
}

#[test]
fn test_decode_all_ones_no_padding() {
    let config = create_config_no_padding();
    let result = decode(&config, b"////");
    assert_eq!(Vec::<u8>::from(result.unwrap()), &[0xFF, 0xFF, 0xFF]);
}

#[test]
fn test_decode_consistency_padded_vs_unpadded() {
    let config_no_pad = create_config_no_padding();
    let config_with_pad = Base64DecodeConfig::new(DECODE_TABLE_BASE64, Some(b'='));

    let unpadded = decode(&config_no_pad, b"SGVsbG8").unwrap();
    let padded = decode(&config_with_pad, b"SGVsbG8=").unwrap();

    assert_eq!(Vec::<u8>::from(unpadded), Vec::<u8>::from(padded));
}

#[test]
fn test_decode_consistency_multi_4() {
    let config_no_pad = create_config_no_padding();
    let config_with_pad = Base64DecodeConfig::new(DECODE_TABLE_BASE64, Some(b'='));

    let unpadded = decode(&config_no_pad, b"Zm9vYmFy").unwrap();
    let padded = decode(&config_with_pad, b"Zm9vYmFy").unwrap();

    assert_eq!(Vec::<u8>::from(unpadded), Vec::<u8>::from(padded));
}

#[test]
fn test_decode_longer_string_no_padding() {
    let config = create_config_no_padding();
    let result = decode(&config, b"SGVsbG8gV29ybGQh");
    assert_eq!(Vec::<u8>::from(result.unwrap()), b"Hello World!");
}

#[test]
fn test_decode_binary_patterns_no_padding() {
    let config = create_config_no_padding();

    let patterns: Vec<(&[u8], &[u8])> = vec![
        (b"AAEC", &[0, 1, 2]),
        (b"AAAA", &[0, 0, 0]),
        (b"////", &[0xFF, 0xFF, 0xFF]),
        (
            b"QUJDREVGR0hJSktMTU5PUFFSU1RVVldYWVowMTIzNDU2Nzg5",
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789",
        ),
    ];

    for (encoded, expected) in patterns {
        let result = decode(&config, encoded).unwrap();
        assert_eq!(
            Vec::<u8>::from(result),
            expected,
            "Failed for {:?}",
            encoded
        );
    }
}
