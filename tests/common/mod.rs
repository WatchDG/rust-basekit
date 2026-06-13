pub mod base16;
pub mod base32;
pub mod base64;

/// Shared 2-byte patterns used by round-trip tests across all bases.
pub const PATTERNS_2B: &[&[u8]] = &[
    &[0x00, 0x00],
    &[0xFF, 0xFF],
    &[0xAA, 0x55],
    &[0x12, 0x34],
    &[0x00, 0x01],
    &[0x80, 0x7F],
    &[0xDE, 0xAD],
    &[0xBE, 0xEF],
];

/// Shared 3-byte patterns used by round-trip tests across all bases.
pub const PATTERNS_3B: &[&[u8]] = &[
    &[0, 0, 0],
    &[0xFF, 0xFF, 0xFF],
    &[0x00, 0xFF, 0x00],
    &[0xAA, 0x55, 0xAA],
    &[1, 2, 3],
    &[255, 254, 253],
];

/// Shared strings used by round-trip tests across all bases.
pub const STRINGS: &[&str] = &[
    "Hello",
    "Hello!",
    "Hello World",
    "Hello, World!",
    "The quick brown fox jumps over the lazy dog",
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",
    "Spaces and\ttabs\nand\nnewlines",
];

/// Deterministic "random-like" data generator used in several tests.
pub fn seed_data(len: usize) -> Vec<u8> {
    (0..len).map(|i| ((i * 17 + 42) % 256) as u8).collect()
}

/// Assert that bytes starting at `len` still contain the marker value.
pub fn assert_untouched(buf: &[u8], marker: u8, len: usize) {
    assert!(
        buf[len..].iter().all(|&b| b == marker),
        "function wrote past returned length: {:?}",
        &buf[len..]
    );
}
