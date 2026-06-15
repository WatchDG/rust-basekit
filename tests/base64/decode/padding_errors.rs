use basekit::base64::{Base64DecodeConfig, Base64Error, DECODE_TABLE_BASE64, decode64};

fn create_config() -> Base64DecodeConfig {
    Base64DecodeConfig::new(DECODE_TABLE_BASE64, Some(b'='))
}

#[test]
fn test_unpadded_decoder_rejects_padding() {
    let config = Base64DecodeConfig::new(DECODE_TABLE_BASE64, None);
    let result = decode64(&config, b"Zg==");
    assert!(
        matches!(result, Err(Base64Error::InvalidCharacter(b'=', pos)) if pos == 2),
        "unpadded decoder should reject '=' as invalid character"
    );
}

#[test]
fn test_data_after_padding_is_rejected() {
    let result = decode64(&create_config(), b"Zg==QQ==");
    assert!(
        matches!(
            result,
            Err(Base64Error::InvalidPadding | Base64Error::InvalidCharacter(_, _))
        ),
        "trailing data after padding should be rejected"
    );
}

#[test]
fn test_too_many_padding_chars() {
    let result = decode64(&create_config(), b"Zg===");
    assert!(
        matches!(result, Err(Base64Error::InvalidPadding)),
        "more than two padding chars should be invalid"
    );
}

#[test]
fn test_full_block_with_padding_is_rejected() {
    // "Zm9v" is a full 3-byte block and should not have padding.
    let result = decode64(&create_config(), b"Zm9v=");
    assert!(
        matches!(result, Err(Base64Error::InvalidPadding)),
        "padding after a full block should be invalid"
    );
}

#[test]
fn test_wrong_padding_count_for_tail() {
    // 1-byte tail uses two padding chars, not one.
    let result = decode64(&create_config(), b"Zg=");
    assert!(
        matches!(result, Err(Base64Error::InvalidPadding)),
        "wrong padding count for tail should be invalid"
    );
}

#[test]
fn test_padding_at_start_of_block() {
    let result = decode64(&create_config(), b"=Zm9v");
    assert!(
        matches!(result, Err(Base64Error::InvalidPadding)),
        "padding at the start of a block should be invalid"
    );
}
