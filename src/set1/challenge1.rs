use super::shared::*;
use std::collections::HashMap;
use std::iter::FromIterator;

// there has to be a better way to do this
fn base64_keys() -> HashMap<u8, char> {
    (('A' as u8)..('Z' as u8 + 1))
        .chain(('a' as u8)..('z' as u8 + 1))
        .chain(('0' as u8)..('9' as u8 + 1))
        .chain(('+' as u8)..('+' as u8 + 1))
        .chain(('/' as u8)..('/' as u8 + 1))
        .enumerate()
        .map(|(k, v)| (k as u8, v as char))
        .collect()
}

fn triplet_to_base64(triplet: u32) -> Vec<char> {
    let base64_chars = base64_keys();
    let mut chars: Vec<char> = Vec::new();
    let mut trip = triplet;

    // we only care about the bottom 3 bytes (4 6-bit values)
    while trip & 0xffffff != 0u32 {
        // base 64 has 64 keys (duh) so each key is 6 bits
        let key = (
            // drop the first 3 6-bit values
            (trip >> 18)
            // and mask off the remaining 6-bit value
            & 0x3f
        ) as u8;
        chars.push(*base64_chars.get(&key).unwrap());
        trip <<= 6;
    }

    while chars.len() < 4 {
        chars.push('=');
    }

    chars
}

pub fn hex_to_base64(hex_str: &str) -> String {
    // this probably shouldn't be a .unwrap()
    let bytes = hex_string_to_bytes(hex_str).unwrap();
    let mut chars: Vec<char> = Vec::new();

    let mut triplet = 0u32;
    let triplet_count = (bytes.len() + 2) / 3;

    for i in 0..(triplet_count * 3) {
        let num = if let Some(&n) = bytes.get(i) { n } else { 0 } as u32;
        triplet = (triplet << 8) + num;

        if i % 3 == 2 {
            for c in triplet_to_base64(triplet).iter() {
                chars.push(*c);
            }
        }
    }

    chars.into_iter().collect()
}

fn nth_byte(num: u32, n: u32) -> u32 {
    num >> (n * 8) & 0xff
}

pub fn base64_to_hex(base64_str: &str) -> String {
    let keys: HashMap<char, u8> = base64_keys().iter().map(|(&k, &v)| (v, k)).collect();
    let chars = Vec::from_iter(base64_str.chars());
    let bytes = chars
        // 4 base64 chars -> 3 bytes
        .chunks(4)
        .map(|quad| {
            quad.iter()
                // collect 4 chars into a single u32 with 3 occupied bytes (a triplet)
                .fold(0, |acc, chr| (acc << 6) + (*keys.get(chr).unwrap_or(&0u8) as u32))
        // split the triplet into individual bytes and flatten
        }).flat_map(|triplet| vec![nth_byte(triplet, 2) as u8, nth_byte(triplet, 1) as u8, triplet as u8])
        .collect();
    bytes_to_hex_string(&bytes).trim_right_matches("0").to_string()
}

#[cfg(test)]
mod test {
    use super::super::shared::bytes_to_hex_string;
    use super::*;

    #[test]
    fn test_hex_to_base64() {
        let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".to_string();
        assert_eq!(expected, hex_to_base64(input));

        let input = bytes_to_hex_string(&b"this is a test".to_vec());
        let expected = "dGhpcyBpcyBhIHRlc3Q=".to_string();
        assert_eq!(expected, hex_to_base64(&input));

        let input = "4d";
        let expected = "TQ==".to_string();
        assert_eq!(expected, hex_to_base64(input));
    }

    #[test]
    fn test_base64_to_hex() {
        let input = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        let expected =
            "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"
                .to_string();
        assert_eq!(expected, base64_to_hex(input));

        let input = "dGhpcyBpcyBhIHRlc3Q=";
        let expected = bytes_to_hex_string(&b"this is a test".to_vec()).to_string();
        assert_eq!(expected, base64_to_hex(&input));

        let input = "TQ==";
        let expected = "4d".to_string();
        assert_eq!(expected, base64_to_hex(input));
    }
}
