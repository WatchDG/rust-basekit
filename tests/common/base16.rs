use basekit::base16::{
    ALPHABET_BASE16_UPPERCASE, Base16DecodeConfig, Base16EncodeConfig,
    DECODE_TABLE_BASE16_UPPERCASE, decode, decode_into, encode, encode_into,
};

fn enc_config() -> Base16EncodeConfig {
    Base16EncodeConfig::new(ALPHABET_BASE16_UPPERCASE)
}

fn dec_config() -> Base16DecodeConfig {
    Base16DecodeConfig::new(DECODE_TABLE_BASE16_UPPERCASE)
}

pub fn roundtrip(original: &[u8]) {
    let encoded = Vec::<u8>::from(encode(&enc_config(), original));
    let decoded = Vec::<u8>::from(decode(&dec_config(), &encoded).unwrap());
    assert_eq!(decoded, original, "Round-trip failed for {:?}", original);
}

pub fn roundtrip_into(original: &[u8]) {
    let enc_len = original.len() * 2;
    let mut enc_dst = vec![0u8; enc_len];
    let actual_enc_len = encode_into(&enc_config(), &mut enc_dst, original).unwrap();
    assert_eq!(actual_enc_len, enc_len);

    let mut dec_dst = vec![0u8; original.len()];
    let actual_dec_len = decode_into(&dec_config(), &mut dec_dst, &enc_dst).unwrap();
    assert_eq!(
        &dec_dst[..actual_dec_len],
        original,
        "Round-trip (into) failed for {:?}",
        original
    );
}
