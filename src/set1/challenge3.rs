use super::challenge2::xor_bytes;
use super::shared::hex_string_to_bytes;
use std::collections::HashMap;
use std::iter;
use std::iter::FromIterator;

#[derive(Debug)]
pub struct Guess {
    key: char,
    message: String,
    weight: u32,
}

// this is a very naive frequency-based character likelihood weighting
const FREQUENCY_LIST: &'static str = " ETAOINSHRDLCUMWFGYPBVKJXQZ";

fn get_weights() -> HashMap<char, u32> {
    FREQUENCY_LIST
        .chars()
        .enumerate()
        .map(|(i, chr)| (chr, i as u32))
        .collect()
}

fn decode(message: &Vec<u8>, key: char) -> Vec<u8> {
    xor_bytes(message, &iter::repeat(key as u8).take(message.len()).collect())
}

fn evaluate(message: &Vec<u8>, weights: &HashMap<char, u32>) -> u32 {
    message
        .iter()
        .map(|chr| weights.get(&(*chr as char).to_ascii_uppercase()).unwrap_or(&27))
        .sum()
}

pub fn guess_xor_message(encoded: &str) -> Guess {
    let bytes = hex_string_to_bytes(encoded).unwrap();
    let weights = get_weights();
    String::from(FREQUENCY_LIST)
        .chars()
        .map(|key| {
            let decoded = decode(&bytes, key);
            Guess {
                key,
                message: String::from_iter(decoded.iter().map(|c| *c as char)),
                weight: evaluate(&decoded, &weights),
            }
        })
        .min_by_key(|guess| guess.weight)
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::guess_xor_message;

    #[test]
    fn guess() {
        assert_eq!(
            "Cooking MC's like a pound of bacon".to_string(),
            guess_xor_message("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736").message
        )
    }
}
