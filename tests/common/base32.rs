use basekit::base32::{
    ALPHABET_BASE32, Base32DecodeConfig, Base32EncodeConfig, DECODE_TABLE_BASE32, decode32,
    decode32_into, encode32, encode32_into,
};

fn enc_config(pad: Option<u8>) -> Base32EncodeConfig {
    Base32EncodeConfig::new(ALPHABET_BASE32, pad)
}

fn dec_config(pad: Option<u8>) -> Base32DecodeConfig {
    Base32DecodeConfig::new(DECODE_TABLE_BASE32, pad)
}

fn encoded_len(len: usize, padded: bool) -> usize {
    let full_groups = len / 5;
    let remainder = len % 5;
    let tail_len = if padded {
        match remainder {
            0 => 0,
            _ => 8,
        }
    } else {
        match remainder {
            0 => 0,
            1 => 2,
            2 => 4,
            3 => 5,
            4 => 7,
            _ => unreachable!(),
        }
    };
    full_groups * 8 + tail_len
}

pub fn roundtrip(original: &[u8]) {
    let encoded = Vec::<u8>::from(encode32(&enc_config(Some(b'=')), original));
    let decoded = Vec::<u8>::from(decode32(&dec_config(Some(b'=')), &encoded).unwrap());
    assert_eq!(decoded, original, "Round-trip failed for {:?}", original);
}

pub fn roundtrip_into(original: &[u8]) {
    let enc_len = encoded_len(original.len(), true);
    let mut enc_dst = vec![0u8; enc_len];
    let actual_enc_len = encode32_into(&enc_config(Some(b'=')), &mut enc_dst, original).unwrap();
    assert_eq!(actual_enc_len, enc_len);

    let mut dec_dst = vec![0u8; original.len()];
    let actual_dec_len = decode32_into(
        &dec_config(Some(b'=')),
        &mut dec_dst,
        &enc_dst[..actual_enc_len],
    )
    .unwrap();
    assert_eq!(
        &dec_dst[..actual_dec_len],
        original,
        "Round-trip (into) failed for {:?}",
        original
    );
}

pub fn roundtrip_no_padding(original: &[u8]) {
    let encoded = Vec::<u8>::from(encode32(&enc_config(None), original));
    let decoded = Vec::<u8>::from(decode32(&dec_config(None), &encoded).unwrap());
    assert_eq!(
        decoded, original,
        "Round-trip no-padding failed for {:?}",
        original
    );
}

pub fn roundtrip_no_padding_into(original: &[u8]) {
    let enc_len = encoded_len(original.len(), false);
    let mut enc_dst = vec![0u8; enc_len];
    let actual_enc_len = encode32_into(&enc_config(None), &mut enc_dst, original).unwrap();
    assert_eq!(actual_enc_len, enc_len);

    let mut dec_dst = vec![0u8; original.len()];
    let actual_dec_len =
        decode32_into(&dec_config(None), &mut dec_dst, &enc_dst[..actual_enc_len]).unwrap();
    assert_eq!(
        &dec_dst[..actual_dec_len],
        original,
        "Round-trip no-padding (into) failed for {:?}",
        original
    );
}

pub fn exact_encode_into_no_padding(data: &[u8]) {
    let config = enc_config(None);
    let expected = Vec::<u8>::from(encode32(&config, data));

    let mut dst = vec![0u8; expected.len()];
    let len = encode32_into(&config, &mut dst, data).unwrap();

    assert_eq!(len, expected.len());
    assert_eq!(&dst[..len], &expected[..]);
}

pub fn exact_decode_into_no_padding(data: &[u8]) {
    let enc = enc_config(None);
    let dec = dec_config(None);

    let encoded = Vec::<u8>::from(encode32(&enc, data));
    let expected = Vec::<u8>::from(decode32(&dec, &encoded).unwrap());

    let mut dst = vec![0u8; expected.len()];
    let len = decode32_into(&dec, &mut dst, &encoded).unwrap();

    assert_eq!(len, expected.len());
    assert_eq!(&dst[..len], &expected[..]);
}
