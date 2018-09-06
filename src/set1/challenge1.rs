use super::shared::hex_string_to_bytes;
use std::collections::HashMap;

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

    chars
}

pub fn hex_to_base64(hex_str: &str) -> String {
    let bytes = hex_string_to_bytes(hex_str).unwrap();
    let mut chars: Vec<char> = Vec::new();

    let mut triplet = 0u32;

    for i in 0..bytes.len() {
        triplet = (triplet << 8) + bytes.get(i).unwrap().clone() as u32;

        if i % 3 == 2 {
            for c in triplet_to_base64(triplet).iter() {
                chars.push(c.clone());
            }
        }
    }

    chars.into_iter().collect()
}

#[cfg(test)]
mod test {
    use super::hex_to_base64;

    #[test]
    fn test_hex_to_base64() {
        let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".to_string();
        assert_eq!(expected, hex_to_base64(input));
    }
}
