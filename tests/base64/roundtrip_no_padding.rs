use crate::common::{
    PATTERNS_2B, PATTERNS_3B, STRINGS,
    base64::{roundtrip_no_padding, roundtrip_no_padding_into},
    seed_data,
};

#[test]
fn test_roundtrip_no_padding_empty() {
    roundtrip_no_padding(&[]);
}

#[test]
fn test_roundtrip_no_padding_strings() {
    for s in STRINGS {
        roundtrip_no_padding(s.as_bytes());
    }
}

#[test]
fn test_roundtrip_no_padding_consistency_with_padded() {
    roundtrip_no_padding(b"Hello, World! The quick brown fox jumps over the lazy dog.");
}

#[test]
fn test_roundtrip_no_padding_binary_patterns() {
    let patterns: Vec<Vec<u8>> = vec![
        (0..64).collect(),
        (0..128).collect(),
        (0..192).collect(),
        (0..255).collect(),
    ];
    for p in patterns {
        roundtrip_no_padding(&p);
    }
}

#[test]
fn test_roundtrip_no_padding_all_zeros() {
    let sizes = [1, 2, 3, 4, 5, 10, 50, 100];
    for size in sizes {
        let data = vec![0u8; size];
        roundtrip_no_padding(&data);
    }
}

#[test]
fn test_roundtrip_no_padding_all_ones() {
    let sizes = [1, 2, 3, 4, 5, 10, 50, 100];
    for size in sizes {
        let data = vec![0xFFu8; size];
        roundtrip_no_padding(&data);
    }
}

#[test]
fn test_roundtrip_no_padding_progressive_sizes() {
    for size in 1..=100 {
        let data: Vec<u8> = (0..size).map(|i| (i % 256) as u8).collect();
        roundtrip_no_padding(&data);
    }
}

#[test]
fn test_roundtrip_no_padding_simd_boundary_sizes() {
    // SIMD encode64 paths process blocks of 12/24/48 input bytes.
    for size in [12, 24, 48] {
        let data: Vec<u8> = seed_data(size);
        roundtrip_no_padding(&data);
        roundtrip_no_padding_into(&data);
    }
}
