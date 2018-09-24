extern crate matasano;

use matasano::set1::challenge3;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut data_file = File::open("./data/s1c4.txt")?;
    let mut data = String::new();
    data_file.read_to_string(&mut data).unwrap();

    let best_guess = data
        .split('\n')
        .map(|line| challenge3::guess_xor_message(line).unwrap())
        .min_by_key(|guess| guess.weight)
        .unwrap();
    println!("{:#?}", best_guess);
    Ok(())
}
