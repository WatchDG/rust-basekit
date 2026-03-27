# Benchmarks

This directory contains performance benchmarks for basekit encoding/decoding functions.

## Structure

```
benches/
├── README.md
├── base16/
│   ├── encode_into_bench/
│   └── decode_into_bench/
├── base32/
│   ├── encode_into_bench/
│   └── decode_into_bench/
└── base64/
    ├── encode_into_bench/
    └── decode_into_bench/
```

## Input Sizes

Tests: **8, 16, 32, 64, 128, 256, 512, 1024 bytes, 1 MB**.

## Base16 encode_into_bench

### Running

```bash
cargo bench --bench base16_encode_into_bench
```

Save baseline:

```bash
cargo bench --bench base16_encode_into_bench -- --save-baseline base16_encode_into_baseline
```

Compare with baseline:

```bash
cargo bench --bench base16_encode_into_bench -- --baseline base16_encode_into_baseline
```

## Base16 decode_into_bench

Test data is prepared using `encode_into` before benchmarking.

### Running

```bash
cargo bench --bench base16_decode_into_bench
```

Save baseline:

```bash
cargo bench --bench base16_decode_into_bench -- --save-baseline base16_decode_into_baseline
```

Compare with baseline:

```bash
cargo bench --bench base16_decode_into_bench -- --baseline base16_decode_into_baseline
```

## Base32 encode_into_bench

### Running

```bash
cargo bench --bench base32_encode_into_bench
```

Save baseline:

```bash
cargo bench --bench base32_encode_into_bench -- --save-baseline base32_encode_into_baseline
```

Compare with baseline:

```bash
cargo bench --bench base32_encode_into_bench -- --baseline base32_encode_into_baseline
```

## Base32 decode_into_bench

Test data is prepared using `encode_into` before benchmarking.

### Running

```bash
cargo bench --bench base32_decode_into_bench
```

Save baseline:

```bash
cargo bench --bench base32_decode_into_bench -- --save-baseline base32_decode_into_baseline
```

Compare with baseline:

```bash
cargo bench --bench base32_decode_into_bench -- --baseline base32_decode_into_baseline
```

## Base64 encode_into_bench

### Running

```bash
cargo bench --bench base64_encode_into_bench
```

Save baseline:

```bash
cargo bench --bench base64_encode_into_bench -- --save-baseline base64_encode_into_baseline
```

Compare with baseline:

```bash
cargo bench --bench base64_encode_into_bench -- --baseline base64_encode_into_baseline
```

## Base64 decode_into_bench

Test data is prepared using `encode_into` before benchmarking.

### Running

```bash
cargo bench --bench base64_decode_into_bench
```

Save baseline:

```bash
cargo bench --bench base64_decode_into_bench -- --save-baseline base64_decode_into_baseline
```

Compare with baseline:

```bash
cargo bench --bench base64_decode_into_bench -- --baseline base64_decode_into_baseline
```
