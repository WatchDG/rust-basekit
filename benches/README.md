# Benchmarks

This directory contains performance benchmarks for basekit encoding/decoding functions.

## Structure

- `base16/` — Base16 (hex) benchmarks
- `base32/` — Base32 benchmarks
- `base64/` — Base64 benchmarks
- `base64/encode_into_bench/` — Standalone benchmark for base64 `encode_into` with variable input sizes

## Base64 encode_into_bench

Standalone benchmark for measuring `encode_into` performance with various input sizes.

### Input Sizes

Tests: **8, 16, 32, 64, 128, 256, 512, 1024 bytes, 1 MB**.

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
cargo bench --bench base64_encode_into_bench -- --baseline base64_encode_into_bench
```
