use std::error::Error;
use std::fs::read_to_string;
use std::iter::FromIterator;

fn main() -> Result<(), Box<Error>> {
    let data = read_to_string("./data/s1c8.txt")?;
    let guess = data
        .trim()
        .split('\n')
        .min_by_key(|line| {
            let mut blocks: Vec<String> = Vec::from_iter(line.chars())
                .chunks(32)
                .map(|chunk| String::from_iter(chunk))
                .collect();
            blocks.sort();
            blocks.dedup();
            blocks.len()
        }).ok_or("Something went wrong")?;
    println!("Guess: {}", guess);
    Ok(())
}
