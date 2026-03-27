use basekit::base32::{Base32DecodeConfig, Base32Error, DECODE_TABLE_BASE32, decode_into};

fn create_config() -> Base32DecodeConfig {
    Base32DecodeConfig::new(DECODE_TABLE_BASE32, Some(b'='))
}

#[test]
fn test_decode_into_exact_buffer() {
    let config = create_config();
    let src = b"MZXW6YTB";
    let mut dst = [0u8; 5];
    let len = decode_into(&config, &mut dst, src).unwrap();
    assert_eq!(len, 5);
    assert_eq!(&dst[..5], b"fooba");
}

#[test]
fn test_decode_into_larger_buffer() {
    let config = create_config();
    let src = b"MY======";
    let mut dst = [0u8; 20];
    let len = decode_into(&config, &mut dst, src).unwrap();
    assert_eq!(len, 1);
    assert_eq!(dst[0], 102);
}

#[test]
fn test_decode_into_small_buffer() {
    let config = create_config();
    let src = b"MZXW6YTB";
    let mut dst = [0u8; 3];
    let result = decode_into(&config, &mut dst, src);
    assert!(matches!(
        result,
        Err(Base32Error::DestinationBufferTooSmall { .. })
    ));
}

#[test]
fn test_decode_into_empty() {
    let config = create_config();
    let mut dst = [0u8; 10];
    let len = decode_into(&config, &mut dst, b"").unwrap();
    assert_eq!(len, 0);
}

#[test]
fn test_decode_into_single_byte() {
    let config = create_config();
    let mut dst = [0u8; 8];
    let len = decode_into(&config, &mut dst, b"MY======").unwrap();
    assert_eq!(len, 1);
    assert_eq!(dst[0], 102);
}
