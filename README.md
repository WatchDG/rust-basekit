# basekit

Universal Rust library for encoding/decoding in various bases.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
basekit = "0.4.1"
```

## Usage

```rust
use basekit::base16::{
    Base16DecodeConfig, Base16EncodeConfig,
    ALPHABET_BASE16_LOWERCASE, DECODE_TABLE_BASE16_LOWERCASE,
    decode as decode16, encode as encode16,
};
use basekit::base32::{
    Base32DecodeConfig, Base32EncodeConfig,
    ALPHABET_BASE32, DECODE_TABLE_BASE32, PADDING_BASE32,
    decode as decode32, encode as encode32,
};
use basekit::base64::{
    Base64DecodeConfig, Base64EncodeConfig,
    ALPHABET_BASE64, ALPHABET_BASE64_URL,
    DECODE_TABLE_BASE64, DECODE_TABLE_BASE64_URL,
    PADDING_BASE64, decode as decode64, encode as encode64,
};

let data = b"Hello, World!";

// Base16
let enc16 = Base16EncodeConfig::new(ALPHABET_BASE16_LOWERCASE);
let dec16 = Base16DecodeConfig::new(DECODE_TABLE_BASE16_LOWERCASE);
let encoded = String::try_from(encode16(&enc16, data)).unwrap();
println!("Base16: {}", encoded);
let decoded = String::try_from(decode16(&dec16, encoded.as_bytes()).unwrap()).unwrap();
println!("Base16 decoded: {}", decoded);

// Base32
let enc32 = Base32EncodeConfig::new(ALPHABET_BASE32, Some(PADDING_BASE32));
let dec32 = Base32DecodeConfig::new(DECODE_TABLE_BASE32, Some(PADDING_BASE32));
let encoded = String::try_from(encode32(&enc32, data)).unwrap();
println!("Base32: {}", encoded);
let decoded = String::try_from(decode32(&dec32, encoded.as_bytes()).unwrap()).unwrap();
println!("Base32 decoded: {}", decoded);

// Base64
let enc64 = Base64EncodeConfig::new(ALPHABET_BASE64, Some(PADDING_BASE64));
let dec64 = Base64DecodeConfig::new(DECODE_TABLE_BASE64, Some(PADDING_BASE64));
let encoded = String::try_from(encode64(&enc64, data)).unwrap();
println!("Base64: {}", encoded);
let decoded = String::try_from(decode64(&dec64, encoded.as_bytes()).unwrap()).unwrap();
println!("Base64 decoded: {}", decoded);

// URL-safe Base64
let enc64url = Base64EncodeConfig::new(ALPHABET_BASE64_URL, Some(PADDING_BASE64));
let dec64url = Base64DecodeConfig::new(DECODE_TABLE_BASE64_URL, Some(PADDING_BASE64));
let encoded = String::try_from(encode64(&enc64url, data)).unwrap();
println!("Base64 URL: {}", encoded);
let decoded = String::try_from(decode64(&dec64url, encoded.as_bytes()).unwrap()).unwrap();
println!("Base64 URL decoded: {}", decoded);
```

## Documentation

Full documentation available at [docs.rs/basekit](https://docs.rs/basekit)
