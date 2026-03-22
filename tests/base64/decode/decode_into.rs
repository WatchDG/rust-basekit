use basekit::base64::{Base64DecodeConfig, Base64Error, DECODE_TABLE_BASE64, decode, decode_into};

fn create_config() -> Base64DecodeConfig {
    Base64DecodeConfig::new(DECODE_TABLE_BASE64, b'=')
}

#[test]
fn test_empty() {
    let config = create_config();
    let mut dst = vec![0u8; 100];
    let len = decode_into(&config, &mut dst, b"").unwrap();
    assert_eq!(len, 0);
}

#[test]
fn test_single_byte() {
    let config = create_config();
    let mut dst = vec![0u8; 100];
    let len = decode_into(&config, &mut dst, b"Zg==").unwrap();
    assert_eq!(len, 1);
    assert_eq!(&dst[..len], &[102]);
}

#[test]
fn test_two_bytes() {
    let config = create_config();
    let mut dst = vec![0u8; 100];
    let len = decode_into(&config, &mut dst, b"Zm8=").unwrap();
    assert_eq!(len, 2);
    assert_eq!(&dst[..len], &[102, 111]);
}

#[test]
fn test_three_bytes() {
    let config = create_config();
    let mut dst = vec![0u8; 100];
    let len = decode_into(&config, &mut dst, b"Zm9v").unwrap();
    assert_eq!(len, 3);
    assert_eq!(&dst[..len], &[102, 111, 111]);
}

#[test]
fn test_four_bytes() {
    let config = create_config();
    let mut dst = vec![0u8; 100];
    let len = decode_into(&config, &mut dst, b"Zm9vYg==").unwrap();
    assert_eq!(len, 4);
    assert_eq!(&dst[..len], &[102, 111, 111, 98]);
}

#[test]
fn test_five_bytes() {
    let config = create_config();
    let mut dst = vec![0u8; 100];
    let len = decode_into(&config, &mut dst, b"Zm9vYmE=").unwrap();
    assert_eq!(len, 5);
    assert_eq!(&dst[..len], &[102, 111, 111, 98, 97]);
}

#[test]
fn test_six_bytes() {
    let config = create_config();
    let mut dst = vec![0u8; 100];
    let len = decode_into(&config, &mut dst, b"Zm9vYmFy").unwrap();
    assert_eq!(len, 6);
    assert_eq!(&dst[..len], &[102, 111, 111, 98, 97, 114]);
}

#[test]
fn test_hello() {
    let config = create_config();
    let mut dst = vec![0u8; 100];
    let len = decode_into(&config, &mut dst, b"SGVsbG8=").unwrap();
    assert_eq!(len, 5);
    assert_eq!(&dst[..len], b"Hello");
}

#[test]
fn test_all_zeros() {
    let config = create_config();
    let mut dst = vec![0u8; 100];
    let len = decode_into(&config, &mut dst, b"AAAA").unwrap();
    assert_eq!(len, 3);
    assert_eq!(&dst[..len], &[0, 0, 0]);
}

#[test]
fn test_all_ones() {
    let config = create_config();
    let mut dst = vec![0u8; 100];
    let len = decode_into(&config, &mut dst, b"////").unwrap();
    assert_eq!(len, 3);
    assert_eq!(&dst[..len], &[0xFF, 0xFF, 0xFF]);
}

#[test]
fn test_invalid_character() {
    let config = create_config();
    let mut dst = vec![0u8; 100];
    let result = decode_into(&config, &mut dst, b"Zm9v!");
    assert!(matches!(result, Err(Base64Error::InvalidCharacter(_, _))));
}

#[test]
fn test_invalid_character_high_byte() {
    let config = create_config();
    let mut dst = vec![0u8; 100];
    let result = decode_into(&config, &mut dst, b"Zm\xFFv");
    assert!(matches!(
        result,
        Err(Base64Error::InvalidCharacter(0xFF, _))
    ));
}

#[test]
fn test_invalid_padding_position() {
    let config = create_config();
    let mut dst = vec![0u8; 100];
    let result = decode_into(&config, &mut dst, b"=Zm9v");
    assert!(matches!(result, Err(Base64Error::InvalidPadding)));
}

#[test]
fn test_too_much_padding() {
    let config = create_config();
    let mut dst = vec![0u8; 100];
    let result = decode_into(&config, &mut dst, b"Zm9v===");
    assert!(matches!(result, Err(Base64Error::InvalidPadding)));
}

#[test]
fn test_invalid_character_at_different_positions() {
    let config = create_config();

    let mut dst = vec![0u8; 100];
    let result = decode_into(&config, &mut dst, b"!m9vYg==");
    assert!(matches!(result, Err(Base64Error::InvalidCharacter(_, 0))));

    let mut dst = vec![0u8; 100];
    let result = decode_into(&config, &mut dst, b"Z!9vYg==");
    assert!(matches!(result, Err(Base64Error::InvalidCharacter(_, 1))));

    let mut dst = vec![0u8; 100];
    let result = decode_into(&config, &mut dst, b"Zm!vYg==");
    assert!(matches!(result, Err(Base64Error::InvalidCharacter(_, 2))));
}

#[test]
fn test_exact_buffer_size() {
    let config = create_config();
    let data = b"SGVsbG8=";
    let output_len = (data.len() / 4) * 3;
    let mut dst = vec![0u8; output_len];
    let len = decode_into(&config, &mut dst, data).unwrap();
    assert_eq!(len, 5);
    assert_eq!(&dst[..len], b"Hello");
}

#[test]
fn test_buffer_too_small_returns_error() {
    let config = create_config();
    let data = b"SGVsbG8=";
    let mut dst = vec![0u8; 3];
    let result = decode_into(&config, &mut dst, data);
    assert!(matches!(
        result,
        Err(Base64Error::DestinationBufferTooSmall {
            needed: 5,
            provided: 3
        })
    ));
}

#[test]
fn test_consistency_with_decode() {
    let config = create_config();
    let data = b"SGVsbG8gV29ybGQh"; // "Hello World!" encoded without padding

    let result = decode(&config, data).unwrap();

    let mut dst = vec![0u8; data.len()];
    let len = decode_into(&config, &mut dst, data).unwrap();

    assert_eq!(len, result.len());
    assert_eq!(&dst[..len], &result[..]);
}
