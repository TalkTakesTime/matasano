fn hex_string_to_bytes(hex_str: &str) -> Option<Vec<u8>> {
    let mut bytes: Vec<u8> = Vec::new();
    let mut byte = 0u8;

    for (i, b) in hex_str.chars().enumerate() {
        match b.to_digit(16) {
            Some(n) => byte += (n << (4 * (1 - (i % 2)))) as u8,
            None => return None,
        }

        if i % 2 == 1 || i == hex_str.len() - 1 {
            bytes.push(byte);
            byte = 0;
        }
    }

    Some(bytes)
}

fn hex_to_base64(hex_str: &str) -> String {
    let bytes = hex_string_to_bytes(hex_str).unwrap();

    
}

#[cfg(test)]
mod test {
    use super::hex_string_to_bytes;

    #[test]
    fn test_hex_string_to_bytes() {
        assert_eq!(vec![1u8], hex_string_to_bytes("01").unwrap());
        assert_eq!(None, hex_string_to_bytes("x"));
        assert_eq!(vec![255u8, 220u8, 15u8], hex_string_to_bytes("ffdc0f").unwrap());
    }

}
