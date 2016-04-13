use std::collections::HashMap;

fn hex_string_to_bytes(hex_str: &str) -> Option<Vec<u8>> {
    let mut bytes: Vec<u8> = Vec::new();
    let mut byte = 0u8;

    let data = if hex_str.len() % 2 == 0 {
        hex_str.to_string()
    } else {
        "0".to_string() + &hex_str
    };

    for (i, b) in data.chars().enumerate() {
        match b.to_digit(16) {
            Some(n) => byte += if i % 2 == 0 {
                n << 4
            } else {
                n
            } as u8,
            None => return None,
        }

        if i % 2 == 1 {
            bytes.push(byte);
            byte = 0;
        }
    }

    Some(bytes)
}

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

fn triplet_to_base64(triplet: &mut u32) -> Vec<char> {
    let keys = base64_keys();
    let mut chars: Vec<char> = Vec::new();

    // haha who likes readable code anyway xd
    while *triplet & 0xffffff != 0u32 {
        chars.push(keys.get(&(((*triplet >> 18) & 0x3f) as u8)).unwrap().clone());
        *triplet <<= 6;
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
            for c in triplet_to_base64(&mut triplet).iter() {
                chars.push(c.clone());
            }
        }
    }

    for c in triplet_to_base64(&mut triplet).iter() {
        chars.push(c.clone());
    }

    chars.into_iter().collect()
}

#[cfg(test)]
mod test {
    use super::{hex_string_to_bytes, hex_to_base64};

    #[test]
    fn test_hex_string_to_bytes() {
        assert_eq!(vec![1u8], hex_string_to_bytes("01").unwrap());
        assert_eq!(vec![12u8], hex_string_to_bytes("c").unwrap());
        assert_eq!(None, hex_string_to_bytes("x"));
        assert_eq!(vec![255u8, 220u8, 15u8], hex_string_to_bytes("ffdc0f").unwrap());
    }

    #[test]
    fn test_hex_to_base64() {
        let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".to_string();
        assert_eq!(expected, hex_to_base64(input));
    }
}
