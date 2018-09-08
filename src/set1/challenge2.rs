use super::shared::*;

pub fn xor_bytes(b1: &Vec<u8>, b2: &Vec<u8>) -> Vec<u8> {
    b1.iter().zip(b2).map(|(x, y)| x ^ y).collect()
}

pub fn fixed_xor(hex1: &str, hex2: &str) -> Result<String, String> {
    let hex1 = hex_string_to_bytes(hex1).unwrap();
    let hex2 = hex_string_to_bytes(hex2).unwrap();

    if hex1.len() != hex2.len() {
        return Err("lengths didn't match".to_string());
    }

    Ok(bytes_to_hex_string(xor_bytes(&hex1, &hex2)))
}

#[cfg(test)]
mod test {
    use super::fixed_xor;

    #[test]
    fn test_fixed_xor() {
        let hex1 = "1c0111001f010100061a024b53535009181c";
        let hex2 = "686974207468652062756c6c277320657965";
        let expected = "746865206b696420646f6e277420706c6179".to_string();

        assert_eq!(expected, fixed_xor(hex1, hex2).unwrap());
    }
}
