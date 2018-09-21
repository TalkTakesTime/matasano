extern crate matasano;

use matasano::set1::challenge3;

fn main() {
    let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    println!("Input: {}", input);
    let guess = challenge3::guess_xor_message(input).unwrap();
    println!("Guessed message: \"{}\"", guess.message());
    println!("Using key '{}', weight {}", guess.key, guess.weight);
}
