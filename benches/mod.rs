pub mod base16;
pub mod base32;
pub mod base64;

criterion::criterion_group!(
    benches,
    base16::encode_benchmarks,
    base16::encode_into_benchmarks,
    base16::decode_benchmarks,
    base16::decode_into_benchmarks,
    base16::roundtrip_benchmarks,
    base32::encode_benchmarks,
    base32::encode_into_benchmarks,
    base32::decode_benchmarks,
    base32::decode_into_benchmarks,
    base32::roundtrip_benchmarks,
    base64::encode_benchmarks,
    base64::encode_into_benchmarks,
    base64::decode_benchmarks,
    base64::decode_into_benchmarks,
    base64::roundtrip_benchmarks
);
criterion::criterion_main!(benches);
