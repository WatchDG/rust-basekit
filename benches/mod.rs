pub mod base64;

criterion::criterion_group!(
    benches,
    base64::encode_benchmarks,
    base64::encode_into_benchmarks,
    base64::decode_benchmarks,
    base64::decode_into_benchmarks,
    base64::roundtrip_benchmarks
);
criterion::criterion_main!(benches);
