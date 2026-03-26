# basekit

Universal Rust library for encoding/decoding in various bases.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
basekit = "0.2.0"
```

## Usage

### Base16

```rust
use basekit::base16::{
    Base16EncodeConfig, Base16DecodeConfig,
    ALPHABET_BASE16_LOWERCASE, DECODE_TABLE_BASE16_LOWERCASE,
    encode, decode,
};

let encode_config = Base16EncodeConfig::new(ALPHABET_BASE16_LOWERCASE);
let decode_config = Base16DecodeConfig::new(DECODE_TABLE_BASE16_LOWERCASE);

let data = b"Hello, World!";
let encoded = encode(&encode_config, data);
let encoded_str = String::try_from(encoded).unwrap();
println!("Encoded: {}", encoded_str);

let decoded = decode(&decode_config, encoded_str.as_bytes()).unwrap();
println!("Decoded: {}", String::from_utf8_lossy(&decoded));
```

### Base32

```rust
use basekit::base32::{
    Base32EncodeConfig, Base32DecodeConfig,
    ALPHABET_BASE32, DECODE_TABLE_BASE32,
    PADDING_BASE32, encode, decode,
};

let encode_config = Base32EncodeConfig::new(ALPHABET_BASE32, PADDING_BASE32);
let decode_config = Base32DecodeConfig::new(DECODE_TABLE_BASE32, PADDING_BASE32);

let data = b"Hello, World!";
let encoded = encode(&encode_config, data);
let encoded_str = String::try_from(encoded).unwrap();
println!("Encoded: {}", encoded_str);

let decoded = decode(&decode_config, encoded_str.as_bytes()).unwrap();
println!("Decoded: {}", String::from_utf8_lossy(&decoded));
```

### Base64

```rust
use basekit::base64::{
    Base64EncodeConfig, Base64DecodeConfig,
    ALPHABET_BASE64, DECODE_TABLE_BASE64,
    PADDING_BASE64, encode, decode,
};

let encode_config = Base64EncodeConfig::new(ALPHABET_BASE64, PADDING_BASE64);
let decode_config = Base64DecodeConfig::new(DECODE_TABLE_BASE64, PADDING_BASE64);

let data = b"Hello, World!";
let encoded = encode(&encode_config, data);
let encoded_str = String::try_from(encoded).unwrap();
println!("Encoded: {}", encoded_str);

let decoded = decode(&decode_config, encoded_str.as_bytes()).unwrap();
println!("Decoded: {}", String::from_utf8_lossy(&decoded));
```

### URL-safe Base64

```rust
use basekit::base64::{
    Base64EncodeConfig, Base64DecodeConfig,
    ALPHABET_BASE64_URL, DECODE_TABLE_BASE64_URL,
    PADDING_BASE64, encode, decode,
};

let encode_config = Base64EncodeConfig::new(ALPHABET_BASE64_URL, PADDING_BASE64);
let decode_config = Base64DecodeConfig::new(DECODE_TABLE_BASE64_URL, PADDING_BASE64);

let data = b"Hello, World!";
let encoded = encode(&encode_config, data);
let encoded_str = String::try_from(encoded).unwrap();
println!("Encoded: {}", encoded_str);

let decoded = decode(&decode_config, encoded_str.as_bytes()).unwrap();
println!("Decoded: {}", String::from_utf8_lossy(&decoded));
```

## Documentation

Full documentation available at [docs.rs/basekit](https://docs.rs/basekit)
