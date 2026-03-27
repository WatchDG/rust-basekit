use basekit::base32::{Base32DecodeConfig, Base32Error, DECODE_TABLE_BASE32_HEX, decode_into};

fn create_config() -> Base32DecodeConfig {
    Base32DecodeConfig::new(DECODE_TABLE_BASE32_HEX, Some(b'='))
}

#[test]
fn test_decode_into_zeros() {
    let config = create_config();
    let src = b"00======";
    let mut dst = [0u8; 10];
    let len = decode_into(&config, &mut dst, src).unwrap();
    assert_eq!(len, 1);
    assert_eq!(dst[0], 0);
}

#[test]
fn test_decode_into_small_buffer() {
    let config = create_config();
    let src = b"00000000";
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
fn test_round_trip() {
    use basekit::base32::{ALPHABET_BASE32_HEX, Base32EncodeConfig, encode_into};
    let decode_config = create_config();
    let encode_config = Base32EncodeConfig::new(ALPHABET_BASE32_HEX, Some(b'='));

    let src = b"Hello, World!";
    let mut encoded = [0u8; 100];
    let len = encode_into(&encode_config, &mut encoded, src).unwrap();
    let encoded = &encoded[..len];

    let mut decoded = [0u8; 100];
    let len = decode_into(&decode_config, &mut decoded, encoded).unwrap();
    assert_eq!(&decoded[..len], src);
}
