extern crate matasano;

use matasano::set1::challenge1::base64_to_hex;
use matasano::set1::challenge7::aes128ecb_decode;
use matasano::set1::shared::hex_string_to_bytes;
use std::error::Error;
use std::fs::read_to_string;
use std::iter::FromIterator;

fn main() -> Result<(), Box<Error>> {
    let data = read_to_string("./data/s1c7.txt")?;
    let hex = base64_to_hex(&data.replace("\n", ""));
    let bytes = hex_string_to_bytes(&hex).ok_or("Could not convert to hex")?;
    println!("found {} bytes", bytes.len());
    let decrypted_data = aes128ecb_decode(b"YELLOW SUBMARINE", &bytes)?;
    println!("{}", String::from_iter(decrypted_data.iter().map(|&c| c as char)));
    Ok(())
}
