use basekit::base32::{Base32DecodeConfig, Base32Error, DECODE_TABLE_BASE32, decode};

fn create_config() -> Base32DecodeConfig {
    Base32DecodeConfig::new(DECODE_TABLE_BASE32, Some(b'='))
}

#[test]
fn test_unpadded_decoder_rejects_padding() {
    let config = Base32DecodeConfig::new(DECODE_TABLE_BASE32, None);
    let result = decode(&config, b"MY======");
    assert!(
        matches!(result, Err(Base32Error::InvalidCharacter(b'=', pos)) if pos == 2),
        "unpadded decoder should reject '=' as invalid character"
    );
}

#[test]
fn test_data_after_padding_is_rejected() {
    let result = decode(&create_config(), b"MY======A");
    assert!(
        matches!(
            result,
            Err(Base32Error::InvalidPadding | Base32Error::InvalidCharacter(_, _))
        ),
        "trailing data after padding should be rejected"
    );
}

#[test]
fn test_full_block_with_padding_is_rejected() {
    // "MZXW6YTB" is a full 5-byte block and should not have padding.
    let result = decode(&create_config(), b"MZXW6YTB=");
    assert!(
        matches!(result, Err(Base32Error::InvalidPadding)),
        "padding after a full block should be invalid"
    );
}

#[test]
fn test_padding_at_start_of_block() {
    let result = decode(&create_config(), b"=MZXW6===");
    assert!(
        matches!(result, Err(Base32Error::InvalidPadding)),
        "padding at the start of a block should be invalid"
    );
}
