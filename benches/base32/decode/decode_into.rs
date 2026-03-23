use basekit::base32::{
    ALPHABET_BASE32, Base32DecodeConfig, Base32EncodeConfig, DECODE_TABLE_BASE32, decode_into,
    encode,
};
use criterion::{BenchmarkId, Criterion, black_box};

fn create_encode_config() -> Base32EncodeConfig {
    Base32EncodeConfig::new(ALPHABET_BASE32, b'=')
}

fn create_decode_config() -> Base32DecodeConfig {
    Base32DecodeConfig::new(DECODE_TABLE_BASE32, b'=')
}

pub fn decode_into_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("decode_into");

    group.bench_function("short", |b| {
        let config = create_decode_config();
        let data = b"JBSWY3DPEBLW64TMMQ======";
        let mut dst = vec![0u8; data.len()];
        b.iter(|| {
            let len =
                decode_into(black_box(&config), black_box(&mut dst), black_box(data)).unwrap();
            black_box(len);
        });
    });

    group.bench_function("medium", |b| {
        let config = create_decode_config();
        let data = b"KR3DIZLQN5XHIZLSNQ======";
        let mut dst = vec![0u8; data.len()];
        b.iter(|| {
            let len =
                decode_into(black_box(&config), black_box(&mut dst), black_box(data)).unwrap();
            black_box(len);
        });
    });

    group.bench_function("large", |b| {
        let config = create_decode_config();
        let data = b"KR3DIFZRNZRW63JRNZSGIYGE2DAMRXM4ZA======";
        let mut dst = vec![0u8; data.len()];
        b.iter(|| {
            let len = decode_into(black_box(&config), black_box(&mut dst), black_box(data)).unwrap();
            black_box(len);
        });
    });

    let size_1kb = 1024;
    group.bench_with_input(BenchmarkId::from_parameter("1kb"), &size_1kb, |b, &size| {
        let encode_config = create_encode_config();
        let decode_config = create_decode_config();
        let original: Vec<u8> = (0..size).map(|i| (i % 256) as u8).collect();
        let encoded = encode(&encode_config, &original);
        let mut dst = vec![0u8; encoded.len()];
        b.iter(|| {
            let len = decode_into(
                black_box(&decode_config),
                black_box(&mut dst),
                black_box(&encoded),
            )
            .unwrap();
            black_box(len);
        });
    });

    let size_1mb = 1024 * 1024;
    group.bench_with_input(BenchmarkId::from_parameter("1mb"), &size_1mb, |b, &size| {
        let encode_config = create_encode_config();
        let decode_config = create_decode_config();
        let original: Vec<u8> = (0..size).map(|i| (i % 256) as u8).collect();
        let encoded = encode(&encode_config, &original);
        let mut dst = vec![0u8; encoded.len()];
        b.iter(|| {
            let len = decode_into(
                black_box(&decode_config),
                black_box(&mut dst),
                black_box(&encoded),
            )
            .unwrap();
            black_box(len);
        });
    });

    group.finish();
}
