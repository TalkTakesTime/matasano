pub fn pkcs7_pad(data: &[u8], block_size: usize) -> Vec<u8> {
    let mut padded_data = Vec::from(data);
    let pad_count = block_size - padded_data.len() % block_size;
    padded_data.extend(vec![pad_count as u8; pad_count]);
    padded_data
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_pad() {
        assert_eq!(
            b"YELLOW SUBMARINE\x04\x04\x04\x04".to_vec(),
            pkcs7_pad(b"YELLOW SUBMARINE", 20)
        );
    }
}
