use basekit::base64::{ALPHABET_BASE64, Base64Config, decode_v1, encode_v1};
use criterion::{BenchmarkId, Criterion, black_box};

fn create_config() -> Base64Config {
    Base64Config::new(ALPHABET_BASE64, b'=')
}

pub fn decode_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("decode");

    group.bench_function("short", |b| {
        let config = create_config();
        let data = b"SGVsbG8sIFdvcmxkIQ==";
        b.iter(|| {
            black_box(decode_v1(black_box(&config), black_box(data)).unwrap());
        });
    });

    group.bench_function("medium", |b| {
        let config = create_config();
        let data = b"VGhlIHF1aWNrIGJyb3duIGZveCBqdW1wcyBvdmVyIHRoZSBsYXp5IGRvZw==";
        b.iter(|| {
            black_box(decode_v1(black_box(&config), black_box(data)).unwrap());
        });
    });

    group.bench_function("large", |b| {
        let config = create_config();
        let data = b"VGhlIHF1aWNrIGJyb3duIGZveCBqdW1wcyBvdmVyIHRoZSBsYXp5IGRvZy4gTG9yZW0gaXBzdW0gZG9sb3Igc2l0IGFtZXQu";
        b.iter(|| {
            black_box(decode_v1(black_box(&config), black_box(data)).unwrap());
        });
    });

    group.bench_with_input(BenchmarkId::from_parameter("1kb"), &1024, |b, &size| {
        let config = create_config();
        let original: Vec<u8> = (0..size).map(|i| (i % 256) as u8).collect();
        let encoded = encode_v1(&config, &original);
        b.iter(|| {
            black_box(decode_v1(black_box(&config), black_box(&encoded)).unwrap());
        });
    });

    group.bench_with_input(
        BenchmarkId::from_parameter("1mb"),
        &(1024 * 1024),
        |b, &size| {
            let config = create_config();
            let original: Vec<u8> = (0..size).map(|i| (i % 256) as u8).collect();
            let encoded = encode_v1(&config, &original);
            b.iter(|| {
                black_box(decode_v1(black_box(&config), black_box(&encoded)).unwrap());
            });
        },
    );

    group.finish();
}

pub fn roundtrip_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("roundtrip");

    group.bench_function("short", |b| {
        let config = create_config();
        let data = b"Hello, World!";
        b.iter(|| {
            let encoded = encode_v1(black_box(&config), black_box(data));
            black_box(decode_v1(black_box(&config), &encoded).unwrap());
        });
    });

    group.bench_function("medium", |b| {
        let config = create_config();
        let data = b"The quick brown fox jumps over the lazy dog";
        b.iter(|| {
            let encoded = encode_v1(black_box(&config), black_box(data));
            black_box(decode_v1(black_box(&config), &encoded).unwrap());
        });
    });

    group.bench_function("large", |b| {
        let config = create_config();
        let data = b"The quick brown fox jumps over the lazy dog. Lorem ipsum dolor sit amet.";
        b.iter(|| {
            let encoded = encode_v1(black_box(&config), black_box(data));
            black_box(decode_v1(black_box(&config), &encoded).unwrap());
        });
    });

    group.finish();
}
