use set1::challenge2::xor_bytes;
use set1::challenge7::Aes128Ecb;

pub struct Aes128Cbc {
    cipher: Aes128Ecb,
}

impl Aes128Cbc {
    pub fn new(key: &[u8]) -> Result<Self, String> {
        let cipher = Aes128Ecb::new(key)?;
        Ok(Self { cipher })
    }

    pub fn encrypt(&mut self, data: &[u8], iv: &[u8]) -> Result<Vec<u8>, String> {
        let mut prev_block = Vec::from(iv);
        if prev_block.len() != 16 {
            return Err("invalid initialization vector length".to_string());
        }

        let mut buf: Vec<u8> = Vec::new();
        let data = Vec::from(data);
        for block in data.chunks(16) {
            let xored_data = xor_bytes(&Vec::from(block), &prev_block);
            let result = self.cipher.encrypt(&xored_data)?;
            buf.extend(result.iter());
            prev_block = result;
        }

        Ok(buf)
    }

    pub fn decrypt(&mut self, data: &[u8], iv: &[u8]) -> Result<Vec<u8>, String> {
        let mut prev_block = Vec::from(iv);
        if prev_block.len() != 16 {
            return Err("invalid initialization vector length".to_string());
        }

        let mut buf: Vec<u8> = Vec::new();
        let data = Vec::from(data);
        for block in data.chunks(16) {
            let result = self.cipher.decrypt(&block)?;
            let xored_data = xor_bytes(&result, &prev_block);
            buf.extend(xored_data);
            prev_block = Vec::from(block);
        }

        Ok(buf)
    }
}
