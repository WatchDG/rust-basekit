use basekit::base64::{ALPHABET_BASE64, Base64EncodeConfig, encode_into};
use criterion::{BenchmarkId, Criterion, Throughput};
use std::hint::black_box;

fn create_config() -> Base64EncodeConfig {
    Base64EncodeConfig::new(ALPHABET_BASE64, Some(b'='))
}

fn main() {
    let mut c = Criterion::default()
        .sample_size(100)
        .measurement_time(std::time::Duration::from_secs(5));

    let mut group = c.benchmark_group("base64_encode_into");

    let small_sizes = [8, 16, 32, 64, 128, 256, 512, 1024];
    let large_sizes = [1024 * 1024];

    for size in small_sizes.iter().chain(large_sizes.iter()) {
        let size = *size;
        group.throughput(Throughput::Bytes(size as u64));

        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let config = create_config();
            let data: Vec<u8> = (0..size).map(|i| (i % 256) as u8).collect();
            let output_size = (size / 3 + 1) * 4;
            let mut dst = vec![0u8; output_size];

            b.iter(|| {
                let len = black_box(
                    encode_into(black_box(&config), black_box(&mut dst), black_box(&data)).unwrap(),
                );
                black_box(len);
            });
        });
    }

    group.finish();
}
