use basekit::base64::{encode_v1, Base64Config, ALPHABET_BASE64};

fn create_config() -> Base64Config {
    Base64Config::new(ALPHABET_BASE64, b'=')
}

#[test]
fn test_empty() {
    let config = create_config();
    let result = encode_v1(&config, &[]);
    assert_eq!(result, b"");
}

#[test]
fn test_single_byte() {
    let config = create_config();
    let result = encode_v1(&config, &[102]); // 'f'
    assert_eq!(result, b"Zg==");
}

#[test]
fn test_two_bytes() {
    let config = create_config();
    let result = encode_v1(&config, &[102, 111]); // 'fo'
    assert_eq!(result, b"Zm8=");
}

#[test]
fn test_three_bytes() {
    let config = create_config();
    let result = encode_v1(&config, &[102, 111, 111]); // 'foo'
    assert_eq!(result, b"Zm9v");
}

#[test]
fn test_four_bytes() {
    let config = create_config();
    let result = encode_v1(&config, &[102, 111, 111, 98]); // 'foob'
    assert_eq!(result, b"Zm9vYg==");
}

#[test]
fn test_five_bytes() {
    let config = create_config();
    let result = encode_v1(&config, &[102, 111, 111, 98, 97]); // 'fooba'
    assert_eq!(result, b"Zm9vYmE=");
}

#[test]
fn test_six_bytes() {
    let config = create_config();
    let result = encode_v1(&config, &[102, 111, 111, 98, 97, 114]); // 'foobar'
    assert_eq!(result, b"Zm9vYmFy");
}

#[test]
fn test_hello() {
    let config = create_config();
    let result = encode_v1(&config, b"Hello");
    assert_eq!(result, b"SGVsbG8=");
}

#[test]
fn test_all_zeros() {
    let config = create_config();
    let result = encode_v1(&config, &[0, 0, 0]);
    assert_eq!(result, b"AAAA");
}

#[test]
fn test_all_ones() {
    let config = create_config();
    let result = encode_v1(&config, &[0xFF, 0xFF, 0xFF]);
    assert_eq!(result, b"////");
}
