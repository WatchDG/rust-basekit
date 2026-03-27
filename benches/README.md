# Benchmarks

This directory contains performance benchmarks for basekit encoding/decoding functions.

## Structure

- `base16/` — Base16 (hex) benchmarks
- `base32/` — Base32 benchmarks
- `base64/` — Base64 benchmarks
- `*/encode_into_bench/` — Standalone benchmarks for `encode_into` with variable input sizes

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
