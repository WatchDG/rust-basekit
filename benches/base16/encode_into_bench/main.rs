use basekit::base16::{ALPHABET_BASE16_LOWERCASE, Base16EncodeConfig, encode_into};
use criterion::{BenchmarkId, Criterion, Throughput};

fn create_config() -> Base16EncodeConfig {
    Base16EncodeConfig::new(ALPHABET_BASE16_LOWERCASE)
}

fn main() {
    let mut c = Criterion::default()
        .sample_size(100)
        .measurement_time(std::time::Duration::from_secs(5));

    let mut group = c.benchmark_group("base16_encode_into");

    let small_sizes = [8, 16, 32, 64, 128, 256, 512, 1024];
    let large_sizes = [1024 * 1024];

    for size in small_sizes.iter().chain(large_sizes.iter()) {
        let size = *size;
        group.throughput(Throughput::Bytes(size as u64));

        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let config = create_config();
            let data: Vec<u8> = (0..size).map(|i| (i % 256) as u8).collect();
            let output_size = size * 2;
            let mut dst = vec![0u8; output_size];

            b.iter(|| {
                let len = criterion::black_box(
                    encode_into(
                        criterion::black_box(&config),
                        criterion::black_box(&mut dst),
                        criterion::black_box(&data),
                    )
                    .unwrap(),
                );
                criterion::black_box(len);
            });
        });
    }

    group.finish();
    c.final_summary();
}
