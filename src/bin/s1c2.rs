extern crate matasano;

use matasano::set1::challenge2;

fn main() {
    let hex1 = "1c0111001f010100061a024b53535009181c";
    let hex2 = "686974207468652062756c6c277320657965";
    println!("Inputs:");
    println!("{}", hex1);
    println!("{}", hex2);
    println!("XOR result:");
    println!("{}", challenge2::fixed_xor(hex1, hex2).unwrap());
}
