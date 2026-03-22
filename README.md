# rust-basekit

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
basekit = "0.1.0"
```

## Usage

```rust
use basekit::{Base64Config, ALPHABET_BASE64, PADDING_BASE64, encode_v1, decode_v1};

let config = Base64Config {
    alphabet: ALPHABET_BASE64,
    padding: PADDING_BASE64,
};

let data = b"Hello, World!";
let encoded = encode_v1(&config, data);
println!("Encoded: {}", String::from_utf8_lossy(&encoded));

let decoded = decode_v1(&config, &encoded).unwrap();
println!("Decoded: {}", String::from_utf8_lossy(&decoded));
```