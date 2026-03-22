use basekit::base64::{ALPHABET_BASE64, Base64EncodeConfig, encode_v1};
use criterion::{BenchmarkId, Criterion, black_box};

fn create_config() -> Base64EncodeConfig {
    Base64EncodeConfig::new(ALPHABET_BASE64, b'=')
}

pub fn encode_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("encode");

    group.bench_function("short", |b| {
        let config = create_config();
        let data = b"Hello, World!";
        b.iter(|| {
            black_box(encode_v1(black_box(&config), black_box(data)));
        });
    });

    group.bench_function("medium", |b| {
        let config = create_config();
        let data = b"The quick brown fox jumps over the lazy dog";
        b.iter(|| {
            black_box(encode_v1(black_box(&config), black_box(data)));
        });
    });

    group.bench_function("large", |b| {
        let config = create_config();
        let data = b"The quick brown fox jumps over the lazy dog. Lorem ipsum dolor sit amet.";
        b.iter(|| {
            black_box(encode_v1(black_box(&config), black_box(data)));
        });
    });

    group.bench_with_input(BenchmarkId::from_parameter("1kb"), &1024, |b, &size| {
        let config = create_config();
        let data: Vec<u8> = (0..size).map(|i| (i % 256) as u8).collect();
        b.iter(|| {
            black_box(encode_v1(black_box(&config), black_box(&data)));
        });
    });

    group.bench_with_input(
        BenchmarkId::from_parameter("1mb"),
        &(1024 * 1024),
        |b, &size| {
            let config = create_config();
            let data: Vec<u8> = (0..size).map(|i| (i % 256) as u8).collect();
            b.iter(|| {
                black_box(encode_v1(black_box(&config), black_box(&data)));
            });
        },
    );

    group.finish();
}
