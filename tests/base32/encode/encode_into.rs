use crate::common::assert_untouched;
use basekit::base32::{ALPHABET_BASE32, Base32EncodeConfig, Base32Error, encode_into};

fn create_config() -> Base32EncodeConfig {
    Base32EncodeConfig::new(ALPHABET_BASE32, Some(b'='))
}

#[test]
fn test_encode_into_exact_buffer() {
    let config = create_config();
    let src = b"fooba";
    let mut dst = [0u8; 8];
    let len = encode_into(&config, &mut dst, src).unwrap();
    assert_eq!(len, 8);
    assert_eq!(&dst, b"MZXW6YTB");
}

#[test]
fn test_encode_into_larger_buffer() {
    let config = create_config();
    let src = b"f";
    let mut dst = [0u8; 20];
    let len = encode_into(&config, &mut dst, src).unwrap();
    assert_eq!(len, 8);
    assert_eq!(&dst[..8], b"MY======");
}

#[test]
fn test_encode_into_small_buffer() {
    let config = create_config();
    let src = b"Hello";
    let mut dst = [0u8; 4];
    let result = encode_into(&config, &mut dst, src);
    assert!(matches!(
        result,
        Err(Base32Error::DestinationBufferTooSmall { .. })
    ));
}

#[test]
fn test_encode_into_empty() {
    let config = create_config();
    let mut dst = [0u8; 10];
    let len = encode_into(&config, &mut dst, &[]).unwrap();
    assert_eq!(len, 0);
}

#[test]
fn test_encode_into_single_byte() {
    let config = create_config();
    let mut dst = [0u8; 8];
    let len = encode_into(&config, &mut dst, &[102]).unwrap();
    assert_eq!(len, 8);
    assert_eq!(&dst, b"MY======");
}

#[test]
fn test_no_write_beyond_returned_length() {
    const MARKER: u8 = 0xCC;

    let config = create_config();
    let mut dst = vec![MARKER; 32];
    let len = encode_into(&config, &mut dst, b"f").unwrap();

    assert_eq!(len, 8);
    assert_eq!(&dst[..len], b"MY======");
    assert_untouched(&dst, MARKER, len);
}
