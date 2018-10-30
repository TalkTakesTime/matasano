extern crate matasano;

use matasano::set1::challenge1::base64_to_hex;
use matasano::set1::shared::{bytes_to_string, hex_string_to_bytes};
use matasano::set2::challenge10::Aes128Cbc;
use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<Error>> {
    let data = read_to_string("./data/s2c10.txt")?;
    let hex = base64_to_hex(&data.replace("\n", ""));
    let bytes = hex_string_to_bytes(&hex).ok_or("Could not convert to hex")?;
    println!("found {} bytes", bytes.len());

    let mut decoder = Aes128Cbc::new(b"YELLOW SUBMARINE")?;
    let result = decoder.decrypt(&bytes, &vec![0u8; 16])?;
    println!("{}", bytes_to_string(&result));

    Ok(())
}
