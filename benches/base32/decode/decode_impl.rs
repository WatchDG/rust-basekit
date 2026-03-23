use basekit::base32::{
    ALPHABET_BASE32, Base32DecodeConfig, Base32EncodeConfig, DECODE_TABLE_BASE32, decode, encode,
};
use criterion::{BenchmarkId, Criterion, black_box};

fn create_encode_config() -> Base32EncodeConfig {
    Base32EncodeConfig::new(ALPHABET_BASE32, b'=')
}

fn create_decode_config() -> Base32DecodeConfig {
    Base32DecodeConfig::new(DECODE_TABLE_BASE32, b'=')
}

pub fn decode_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("decode");

    group.bench_function("short", |b| {
        let config = create_decode_config();
        let data = b"JBSWY3DPEBLW64TMMQ======";
        b.iter(|| {
            black_box(decode(black_box(&config), black_box(data)).unwrap());
        });
    });

    group.bench_function("medium", |b| {
        let config = create_decode_config();
        let data = b"KR3DIZLQN5XHIZLSNQ======";
        b.iter(|| {
            black_box(decode(black_box(&config), black_box(data)).unwrap());
        });
    });

    group.bench_function("large", |b| {
        let config = create_decode_config();
        let data = b"KR3DIFZRNZRW63JRNZSGIYGE2DAMRXM4ZA======";
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
