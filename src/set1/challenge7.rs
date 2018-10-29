use aes::Aes128;
use block_modes::block_padding::ZeroPadding;
use block_modes::{BlockMode, Ecb};

type Aes128Ecb = Ecb<Aes128, ZeroPadding>;

pub fn aes128ecb_decode(key: &[u8], data: &[u8]) -> Result<Vec<u8>, String> {
    let cipher = match Aes128Ecb::new_varkey(key) {
        Ok(v) => v,
        Err(_) => return Err("invalid key length".to_string()),
    };
    let mut buf = Vec::from(data);
    match cipher.decrypt_pad(&mut buf) {
        Ok(decrypted_data) => Ok(Vec::from(decrypted_data)),
        Err(_) => Err("failed to decrypt".to_string()),
    }
}
