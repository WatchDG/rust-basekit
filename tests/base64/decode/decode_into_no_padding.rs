use basekit::base64::{Base64DecodeConfig, DECODE_TABLE_BASE64, decode, decode_into};

fn create_config_no_padding() -> Base64DecodeConfig {
    Base64DecodeConfig::new(DECODE_TABLE_BASE64, None)
}

#[test]
fn test_decode_into_empty() {
    let config = create_config_no_padding();
    let mut dst = vec![0u8; 100];
    let len = decode_into(&config, &mut dst, b"").unwrap();
    assert_eq!(len, 0);
}

#[test]
fn test_decode_into_consistency_with_decode() {
    let config = create_config_no_padding();
    let data = b"SGVsbG8gV29ybGQh";

    let result = Vec::<u8>::from(decode(&config, data).unwrap());

    let mut dst = vec![0u8; result.len() + 10];
    let len = decode_into(&config, &mut dst, data).unwrap();

    assert_eq!(len, result.len());
    assert_eq!(&dst[..len], &result[..]);
}

#[test]
fn test_decode_into_consistency_padded_vs_unpadded() {
    let config_no_pad = create_config_no_padding();
    let config_with_pad = Base64DecodeConfig::new(DECODE_TABLE_BASE64, Some(b'='));

    let unpadded = decode(&config_no_pad, b"SGVsbG8").unwrap();
    let padded = decode(&config_with_pad, b"SGVsbG8=").unwrap();

    assert_eq!(Vec::<u8>::from(unpadded), Vec::<u8>::from(padded));
}
