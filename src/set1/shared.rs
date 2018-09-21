pub fn hex_string_to_bytes(hex_str: &str) -> Option<Vec<u8>> {
    let mut bytes: Vec<u8> = Vec::new();
    let mut byte = 0u8;

    let data = if hex_str.len() % 2 == 0 {
        hex_str.to_string()
    } else {
        "0".to_string() + &hex_str
    };

    for (i, b) in data.chars().enumerate() {
        match b.to_digit(16) {
            Some(n) => byte += if i % 2 == 0 { n << 4 } else { n } as u8,
            None => return None,
        }

        if i % 2 == 1 {
            bytes.push(byte);
            byte = 0;
        }
    }

    Some(bytes)
}

pub fn bytes_to_hex_string(bytes: &Vec<u8>) -> String {
    if bytes.iter().all(|x| *x == 0u8) {
        "0".to_string()
    } else {
        bytes
            .iter()
            .map(|x| format!("{:02x}", x))
            .collect::<Vec<String>>()
            .concat()
            .to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hex_string_to_bytes() {
        assert_eq!(vec![1u8], hex_string_to_bytes("01").unwrap());
        assert_eq!(vec![12u8], hex_string_to_bytes("c").unwrap());
        assert_eq!(None, hex_string_to_bytes("x"));
        assert_eq!(vec![255u8, 220u8, 15u8], hex_string_to_bytes("ffdc0f").unwrap());
    }

    #[test]
    fn test_bytes_to_hex_string() {
        assert_eq!("ff".to_string(), bytes_to_hex_string(&vec![255u8]));
        assert_eq!("0".to_string(), bytes_to_hex_string(&vec![0u8]));
        assert_eq!("ffdc0f".to_string(), bytes_to_hex_string(&vec![255u8, 220u8, 15u8]));
        assert_eq!("0c".to_string(), bytes_to_hex_string(&vec![12u8]));
    }
}
