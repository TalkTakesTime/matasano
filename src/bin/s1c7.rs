extern crate matasano;

use matasano::set1::challenge1::base64_to_hex;
use matasano::set1::challenge7::Aes128Ecb;
use matasano::set1::shared::{bytes_to_string, hex_string_to_bytes};
use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<Error>> {
    let data = read_to_string("./data/s1c7.txt")?;
    let hex = base64_to_hex(&data.replace("\n", ""));
    let bytes = hex_string_to_bytes(&hex).ok_or("Could not convert to hex")?;
    println!("found {} bytes", bytes.len());
    let mut encoder = Aes128Ecb::new(b"YELLOW SUBMARINE")?;
    let decrypted_data = encoder.decrypt(&bytes)?;
    println!("{}", bytes_to_string(&decrypted_data));
    Ok(())
}
