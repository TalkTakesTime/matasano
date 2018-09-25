use super::challenge2::xor_bytes;
use super::shared::hex_string_to_bytes;
use std::collections::HashMap;
use std::iter;
use std::iter::FromIterator;

#[derive(Debug)]
pub struct Guess {
    pub key: u8,
    pub data: Vec<u8>,
    pub weight: usize,
}

impl Guess {
    pub fn message(&self) -> String {
        String::from_iter(self.data.iter().map(|&c| c as char))
    }
}

static FREQUENCY_LIST: &str = " ETAOINSHRDLCUMWFGYPBVKJXQZ";

pub fn get_weights() -> HashMap<char, usize> {
    FREQUENCY_LIST
        .chars()
        .enumerate()
        .map(|(i, chr)| (chr, i as usize))
        .collect()
}

fn decode(message: &Vec<u8>, key: u8) -> Vec<u8> {
    xor_bytes(message, &iter::repeat(key as u8).take(message.len()).collect())
}

fn evaluate(message: &Vec<u8>, weights: &HashMap<char, usize>) -> usize {
    message
        .iter()
        .map(|&chr| weights.get(&(chr as char).to_ascii_uppercase()).unwrap_or(&27))
        .sum()
}

pub fn guess_xor_bytes(encoded: &Vec<u8>, weights: &HashMap<char, usize>) -> Option<Guess> {
    (1u8..255u8)
        .map(|key| {
            let decoded = decode(&encoded, key);
            Guess {
                key: key,
                weight: evaluate(&decoded, &weights),
                data: decoded,
            }
        }).min_by_key(|guess| guess.weight)
}

pub fn guess_xor_message(encoded: &str) -> Option<Guess> {
    let bytes = hex_string_to_bytes(encoded)?;
    let weights = get_weights();
    guess_xor_bytes(&bytes, &weights)
}

#[cfg(test)]
mod test {
    use super::guess_xor_message;

    #[test]
    fn guess() {
        assert_eq!(
            "Cooking MC's like a pound of bacon".to_string(),
            guess_xor_message("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")
                .unwrap()
                .message()
        )
    }
}
