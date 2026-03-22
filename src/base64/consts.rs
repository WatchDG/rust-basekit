pub const ALPHABET_BASE64: &[u8; 64] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
pub const ALPHABET_BASE64_URL: &[u8; 64] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

pub const PADDING_BASE64: u8 = b'=';

const fn generate_decode_table(alphabet: &[u8; 64]) -> [i8; 128] {
    let mut table = [-1i8; 128];
    let mut i = 0;
    while i < 64 {
        table[alphabet[i] as usize] = i as i8;
        i += 1;
    }
    table
}

pub const DECODE_TABLE_BASE64: &[i8; 128] = &generate_decode_table(ALPHABET_BASE64);
pub const DECODE_TABLE_BASE64_URL: &[i8; 128] = &generate_decode_table(ALPHABET_BASE64_URL);
