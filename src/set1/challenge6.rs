pub fn hamming_distance(str1: &str, str2: &str) -> u32 {
    str1.bytes()
        .zip(str2.bytes())
        .map(|(b1, b2)| (b1 ^ b2).count_ones())
        .sum()
}

#[cfg(test)]
mod test {
    use super::hamming_distance;

    #[test]
    fn test_hamming_distance() {
        assert_eq!(hamming_distance("this is a test", "wokka wokka!!!"), 37);
    }
}
