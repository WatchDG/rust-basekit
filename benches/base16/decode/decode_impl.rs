use basekit::base16::{
    ALPHABET_BASE16_LOWERCASE, Base16DecodeConfig, Base16EncodeConfig,
    DECODE_TABLE_BASE16_LOWERCASE, decode, encode,
};
use criterion::{BenchmarkId, Criterion, black_box};

fn create_encode_config() -> Base16EncodeConfig {
    Base16EncodeConfig::new(ALPHABET_BASE16_LOWERCASE)
}

fn create_decode_config() -> Base16DecodeConfig {
    Base16DecodeConfig::new(DECODE_TABLE_BASE16_LOWERCASE)
}

pub fn decode_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("decode");

    group.bench_function("short", |b| {
        let config = create_decode_config();
        let data = b"48656c6c6f2c20576f726c6421";
        b.iter(|| {
            black_box(decode(black_box(&config), black_box(data)).unwrap());
        });
    });

    group.bench_function("medium", |b| {
        let config = create_decode_config();
        let data = b"54686520717569636b2062726f776e20666f78206a756d7073206f76657220746865206c617a7920646f67";
        b.iter(|| {
            black_box(decode(black_box(&config), black_box(data)).unwrap());
        });
    });

    group.bench_function("large", |b| {
        let config = create_decode_config();
        let data = b"54686520717569636b2062726f776e20666f78206a756d7073206f76657220746865206c617a7920646f672e204c6f72656d20697073756d20646f6c6f722073697420616d65742e";
        b.iter(|| {
            black_box(decode(black_box(&config), black_box(data)).unwrap());
        });
    });

    let size_1kb = 1024;
    group.bench_with_input(BenchmarkId::from_parameter("1kb"), &size_1kb, |b, &size| {
        let encode_config = create_encode_config();
        let decode_config = create_decode_config();
        let original: Vec<u8> = (0..size).map(|i| (i % 256) as u8).collect();
        let encoded = encode(&encode_config, &original);
        b.iter(|| {
            black_box(decode(black_box(&decode_config), black_box(&encoded)).unwrap());
        });
    });

    let size_1mb = 1024 * 1024;
    group.bench_with_input(BenchmarkId::from_parameter("1mb"), &size_1mb, |b, &size| {
        let encode_config = create_encode_config();
        let decode_config = create_decode_config();
        let original: Vec<u8> = (0..size).map(|i| (i % 256) as u8).collect();
        let encoded = encode(&encode_config, &original);
        b.iter(|| {
            black_box(decode(black_box(&decode_config), black_box(&encoded)).unwrap());
        });
    });

    group.finish();
}

pub fn roundtrip_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("roundtrip");

    group.bench_function("short", |b| {
        let encode_config = create_encode_config();
        let decode_config = create_decode_config();
        let data = b"Hello, World!";
        b.iter(|| {
            let encoded = encode(black_box(&encode_config), black_box(data));
            black_box(decode(black_box(&decode_config), &encoded).unwrap());
        });
    });

    group.bench_function("medium", |b| {
        let encode_config = create_encode_config();
        let decode_config = create_decode_config();
        let data = b"The quick brown fox jumps over the lazy dog";
        b.iter(|| {
            let encoded = encode(black_box(&encode_config), black_box(data));
            black_box(decode(black_box(&decode_config), &encoded).unwrap());
        });
    });

    group.bench_function("large", |b| {
        let encode_config = create_encode_config();
        let decode_config = create_decode_config();
        let data = b"The quick brown fox jumps over the lazy dog. Lorem ipsum dolor sit amet.";
        b.iter(|| {
            let encoded = encode(black_box(&encode_config), black_box(data));
            black_box(decode(black_box(&decode_config), &encoded).unwrap());
        });
    });

    group.finish();
}
