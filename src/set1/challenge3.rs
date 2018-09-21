use super::challenge2::xor_bytes;
use super::shared::hex_string_to_bytes;
use std::iter;
use std::iter::FromIterator;

#[derive(Debug)]
pub struct Guess {
    pub key: u8,
    pub message: String,
    pub weight: usize,
}

fn decode(message: &Vec<u8>, key: u8) -> Vec<u8> {
    xor_bytes(message, &iter::repeat(key as u8).take(message.len()).collect())
}

fn evaluate(message: &Vec<u8>) -> usize {
    message
        .iter()
        .filter(|&&x| (x as char).is_ascii_alphabetic() || x == b' ')
        .count()
}

pub fn guess_xor_message(encoded: &str) -> Option<Guess> {
    let bytes = hex_string_to_bytes(encoded)?;
    (1u8..255u8)
        .map(|key| {
            let decoded = decode(&bytes, key);
            Guess {
                key: key,
                weight: evaluate(&decoded),
                message: String::from_iter(decoded.iter().map(|c| *c as char)),
            }
        }).max_by_key(|guess| guess.weight)
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
                .message
        )
    }
}
