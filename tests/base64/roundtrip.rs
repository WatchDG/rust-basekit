use crate::common::{
    PATTERNS_2B, PATTERNS_3B, STRINGS,
    base64::{roundtrip, roundtrip_into},
    seed_data,
};

#[test]
fn test_roundtrip_empty() {
    roundtrip(&[]);
    roundtrip_into(&[]);
}

#[test]
fn test_roundtrip_single_byte() {
    for i in 0u8..=255 {
        roundtrip(&[i]);
        roundtrip_into(&[i]);
    }
}

#[test]
fn test_roundtrip_two_bytes() {
    for p in PATTERNS_2B {
        roundtrip(p);
        roundtrip_into(p);
    }
}

#[test]
fn test_roundtrip_three_bytes() {
    for p in PATTERNS_3B {
        roundtrip(p);
        roundtrip_into(p);
    }
}

#[test]
fn test_roundtrip_four_bytes() {
    roundtrip(b"foo");
    roundtrip_into(b"foo");

    roundtrip(b"bar");
    roundtrip_into(b"bar");

    roundtrip(b"test");
    roundtrip_into(b"test");
}

#[test]
fn test_roundtrip_strings() {
    for s in STRINGS {
        roundtrip(s.as_bytes());
        roundtrip_into(s.as_bytes());
    }
}

#[test]
fn test_roundtrip_binary_patterns() {
    let patterns: Vec<Vec<u8>> = vec![
        (0..64).collect(),
        (0..128).collect(),
        (0..192).collect(),
        (0..255).collect(),
        (0..100).step_by(1).collect(),
        (0..100).step_by(2).collect(),
        (0..100).step_by(3).collect(),
        (0..100).step_by(7).collect(),
        (0..100).step_by(13).collect(),
        (0..100).step_by(17).collect(),
    ];
    for p in patterns {
        roundtrip(&p);
        roundtrip_into(&p);
    }
}

#[test]
fn test_roundtrip_all_zeros() {
    let sizes = [1, 2, 3, 4, 5, 10, 50, 100, 255, 256, 1000];
    for size in sizes {
        let data = vec![0u8; size];
        roundtrip(&data);
        roundtrip_into(&data);
    }
}

#[test]
fn test_roundtrip_all_ones() {
    let sizes = [1, 2, 3, 4, 5, 10, 50, 100, 255, 256, 1000];
    for size in sizes {
        let data = vec![0xFFu8; size];
        roundtrip(&data);
        roundtrip_into(&data);
    }
}

#[test]
fn test_roundtrip_alternating_pattern() {
    let sizes = [1, 2, 3, 4, 5, 10, 50, 100, 255, 256];
    for size in sizes {
        let data: Vec<u8> = (0..size)
            .map(|i| if i % 2 == 0 { 0xAA } else { 0x55 })
            .collect();
        roundtrip(&data);
        roundtrip_into(&data);
    }
}

#[test]
fn test_roundtrip_random_like() {
    let seed = seed_data(1000);

    for size in [1, 2, 3, 4, 5, 10, 50, 100, 255, 256, 500, 1000] {
        let data: Vec<u8> = seed[..size].to_vec();
        roundtrip(&data);
        roundtrip_into(&data);
    }
}

#[test]
fn test_roundtrip_progressive_sizes() {
    for size in 1..=100 {
        let data: Vec<u8> = (0..size).map(|i| (i % 256) as u8).collect();
        roundtrip(&data);
        roundtrip_into(&data);
    }
}

#[test]
fn test_roundtrip_all_sizes_1_to_30() {
    for size in 1..=30 {
        let data: Vec<u8> = (0..size).map(|i| ((i * 7 + 13) % 256) as u8).collect();
        roundtrip(&data);
        roundtrip_into(&data);
    }
}
