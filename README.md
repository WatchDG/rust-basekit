# basekit

Universal Rust library for encoding/decoding in various bases.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
basekit = "0.2.0"
```

## Usage

```rust
use basekit::base64::{
    Base64EncodeConfig, Base64DecodeConfig, ALPHABET_BASE64, DECODE_TABLE_BASE64,
    PADDING_BASE64, encode_v1, decode_v1,
};

let encode_config = Base64EncodeConfig::new(ALPHABET_BASE64, PADDING_BASE64);
let decode_config = Base64DecodeConfig::new(DECODE_TABLE_BASE64, PADDING_BASE64);

let data = b"Hello, World!";
let encoded = encode_v1(&encode_config, data);
println!("Encoded: {}", String::from_utf8_lossy(&encoded));

let decoded = decode_v1(&decode_config, &encoded).unwrap();
println!("Decoded: {}", String::from_utf8_lossy(&decoded));
```

### URL-safe Base64

```rust
use basekit::base64::{
    Base64EncodeConfig, Base64DecodeConfig, ALPHABET_BASE64_URL,
    DECODE_TABLE_BASE64_URL, PADDING_BASE64, encode_v1, decode_v1,
};

let encode_config = Base64EncodeConfig::new(ALPHABET_BASE64_URL, PADDING_BASE64);
let decode_config = Base64DecodeConfig::new(DECODE_TABLE_BASE64_URL, PADDING_BASE64);

let data = b"Hello, World!";
let encoded = encode_v1(&encode_config, data);
let decoded = decode_v1(&decode_config, &encoded).unwrap();
```

## Documentation

Full documentation available at [docs.rs/basekit](https://docs.rs/basekit)