use super::challenge2::xor_bytes;

pub fn encode(data: &Vec<u8>, key: &Vec<u8>) -> Vec<u8> {
    xor_bytes(data, &key.iter().cloned().cycle().take(data.len()).collect())
}

#[cfg(test)]
mod test {
    use super::super::shared::bytes_to_hex_string;
    use super::encode;

    #[test]
    fn test_encode() {
        let input = b"Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal";
        let output = String::from("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f");
        assert_eq!(bytes_to_hex_string(&encode(&input.to_vec(), &b"ICE".to_vec())), output);
    }
}
