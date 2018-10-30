use aes::Aes128;
use block_modes::block_padding::ZeroPadding;
use block_modes::{BlockMode, Ecb};

pub struct Aes128Ecb {
    cipher: Ecb<Aes128, ZeroPadding>,
}

impl Aes128Ecb {
    pub fn new(key: &[u8]) -> Result<Self, String> {
        match Ecb::<Aes128, ZeroPadding>::new_varkey(key) {
            Ok(cipher) => Ok(Self { cipher }),
            Err(_) => Err("invalid key length".to_string()),
        }
    }

    pub fn decrypt(&mut self, data: &[u8]) -> Result<Vec<u8>, String> {
        let mut buf = Vec::from(data);
        match self.cipher.decrypt_nopad(&mut buf) {
            Ok(()) => Ok(buf),
            Err(_) => Err("failed to decrypt".to_string()),
        }
    }

    pub fn encrypt(&mut self, data: &[u8]) -> Result<Vec<u8>, String> {
        let mut buf = Vec::from(data);
        match self.cipher.encrypt_nopad(&mut buf) {
            Ok(()) => Ok(buf),
            Err(_) => Err("failed to encrypt".to_string()),
        }
    }
}
