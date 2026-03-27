use basekit::base32::{
    ALPHABET_BASE32, Base32DecodeConfig, Base32EncodeConfig, DECODE_TABLE_BASE32, decode_into,
    encode_into,
};
use criterion::{BenchmarkId, Criterion, Throughput};

fn create_encode_config() -> Base32EncodeConfig {
    Base32EncodeConfig::new(ALPHABET_BASE32, b'=')
}

fn create_decode_config() -> Base32DecodeConfig {
    Base32DecodeConfig::new(DECODE_TABLE_BASE32, b'=')
}

fn main() {
    let mut c = Criterion::default()
        .sample_size(100)
        .measurement_time(std::time::Duration::from_secs(5));

    let mut group = c.benchmark_group("base32_decode_into");

    let small_sizes = [8, 16, 32, 64, 128, 256, 512, 1024];
    let large_sizes = [1024 * 1024];

    for size in small_sizes.iter().chain(large_sizes.iter()) {
        let size = *size;
        group.throughput(Throughput::Bytes(size as u64));

        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let encode_config = create_encode_config();
            let decode_config = create_decode_config();

            let data: Vec<u8> = (0..size).map(|i| (i % 256) as u8).collect();
            let encoded_size = (size / 5 + 1) * 8;
            let mut encoded = vec![0u8; encoded_size];
            encode_into(&encode_config, &mut encoded, &data).unwrap();

            let mut dst = vec![0u8; size];

            b.iter(|| {
                let len = criterion::black_box(
                    decode_into(
                        criterion::black_box(&decode_config),
                        criterion::black_box(&mut dst),
                        criterion::black_box(&encoded),
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
