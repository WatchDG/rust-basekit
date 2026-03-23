pub const ALPHABET_BASE32: &[u8; 32] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";

pub const ALPHABET_BASE32_HEX: &[u8; 32] = b"0123456789ABCDEFGHIJKLMNOPQRSTUV";

pub const PADDING_BASE32: u8 = b'=';

const fn generate_decode_table(alphabet: &[u8; 32]) -> [i8; 128] {
    let mut table = [-1i8; 128];
    let mut i = 0;
    while i < 32 {
        table[alphabet[i] as usize] = i as i8;
        i += 1;
    }
    table
}

pub const DECODE_TABLE_BASE32: &[i8; 128] = &generate_decode_table(ALPHABET_BASE32);

pub const DECODE_TABLE_BASE32_HEX: &[i8; 128] = &generate_decode_table(ALPHABET_BASE32_HEX);
