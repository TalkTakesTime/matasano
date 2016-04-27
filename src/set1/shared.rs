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

pub fn bytes_to_hex_string(bytes: Vec<u8>) -> String {
    bytes.iter()
        .map(|x| format!("{:x}", x))
        .collect::<Vec<String>>()
        .concat()
        .to_string()
}
