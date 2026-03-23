use basekit::base32::{ALPHABET_BASE32_HEX, Base32EncodeConfig, Base32Error, encode_into};

fn create_config() -> Base32EncodeConfig {
    Base32EncodeConfig::new(ALPHABET_BASE32_HEX, b'=')
}

#[test]
fn test_encode_into_zeros() {
    let config = create_config();
    let src = [0u8; 1];
    let mut dst = [0u8; 8];
    let len = encode_into(&config, &mut dst, &src).unwrap();
    assert_eq!(len, 8);
    assert_eq!(&dst, b"00======");
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
fn test_round_trip() {
    use basekit::base32::{Base32DecodeConfig, DECODE_TABLE_BASE32_HEX, decode_into};
    let encode_config = create_config();
    let decode_config = Base32DecodeConfig::new(DECODE_TABLE_BASE32_HEX, b'=');

    let src = b"Hello, World!";
    let mut encoded = [0u8; 100];
    let len = encode_into(&encode_config, &mut encoded, src).unwrap();
    let encoded = &encoded[..len];

    let mut decoded = [0u8; 100];
    let len = decode_into(&decode_config, &mut decoded, encoded).unwrap();
    assert_eq!(&decoded[..len], src);
}
