extern crate matasano;

use matasano::set1::challenge1::base64_to_hex;
use matasano::set1::challenge6;
use matasano::set1::shared::hex_string_to_bytes;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::iter::FromIterator;

fn main() -> io::Result<()> {
    let mut data_file = File::open("./data/s1c6.txt")?;
    let mut data = String::new();
    data_file.read_to_string(&mut data).unwrap();

    let hex = base64_to_hex(&data.replace("\n", ""));
    let bytes = hex_string_to_bytes(&hex).unwrap();
    let (key, decrypted_data) = challenge6::decrypt_xor_cipher(&bytes);
    println!(
        "Guess: {}",
        String::from_iter(decrypted_data.iter().map(|&c| c as char))
    );
    println!("Key: {:?}", key);
    Ok(())
}
