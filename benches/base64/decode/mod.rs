pub mod decode_into;
pub mod v1;

pub use decode_into::decode_into_benchmarks;
pub use v1::{decode_benchmarks, roundtrip_benchmarks};
