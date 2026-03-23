use basekit::base16::{ALPHABET_BASE16_LOWERCASE, Base16EncodeConfig, encode_into};
use criterion::{BenchmarkId, Criterion, black_box};

fn create_config() -> Base16EncodeConfig {
    Base16EncodeConfig::new(ALPHABET_BASE16_LOWERCASE)
}

pub fn encode_into_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("encode_into");

    group.bench_function("short", |b| {
        let config = create_config();
        let data = b"Hello, World!";
        let mut dst = vec![0u8; 100];
        b.iter(|| {
            let len = encode_into(black_box(&config), black_box(&mut dst), black_box(data));
            black_box(len);
        });
    });

    group.bench_function("medium", |b| {
        let config = create_config();
        let data = b"The quick brown fox jumps over the lazy dog";
        let mut dst = vec![0u8; 100];
        b.iter(|| {
            let len = encode_into(black_box(&config), black_box(&mut dst), black_box(data));
            black_box(len);
        });
    });

    group.bench_function("large", |b| {
        let config = create_config();
        let data = b"The quick brown fox jumps over the lazy dog. Lorem ipsum dolor sit amet.";
        let mut dst = vec![0u8; 200];
        b.iter(|| {
            let len = encode_into(black_box(&config), black_box(&mut dst), black_box(data));
            black_box(len);
        });
    });

    group.bench_with_input(BenchmarkId::from_parameter("1kb"), &1024, |b, &size| {
        let config = create_config();
        let data: Vec<u8> = (0..size).map(|i| (i % 256) as u8).collect();
        let mut dst = vec![0u8; size * 2 + 10];
        b.iter(|| {
            let len = encode_into(black_box(&config), black_box(&mut dst), black_box(&data));
            black_box(len);
        });
    });

    group.bench_with_input(
        BenchmarkId::from_parameter("1mb"),
        &(1024 * 1024),
        |b, &size| {
            let config = create_config();
            let data: Vec<u8> = (0..size).map(|i| (i % 256) as u8).collect();
            let mut dst = vec![0u8; size * 2 + 10];
            b.iter(|| {
                let len = encode_into(black_box(&config), black_box(&mut dst), black_box(&data));
                black_box(len);
            });
        },
    );

    group.finish();
}
