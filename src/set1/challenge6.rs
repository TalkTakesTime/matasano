use super::challenge3::{get_weights, guess_xor_bytes};
use super::challenge5::encode;

fn hamming_distance<'a>(bytes1: impl Iterator<Item = &'a u8>, bytes2: impl Iterator<Item = &'a u8>) -> u32 {
    bytes1.zip(bytes2).map(|(&b1, &b2)| (b1 ^ b2).count_ones()).sum()
}

fn normalised_hamming_distance<'a>(
    bytes1: impl Iterator<Item = &'a u8>,
    bytes2: impl Iterator<Item = &'a u8>,
    count: usize,
) -> f64 {
    hamming_distance(bytes1.take(count), bytes2.take(count)) as f64 / count as f64
}

fn n_block_average_dist(data: &Vec<u8>, block_count: usize, block_size: usize) -> f64 {
    let distance_count = block_count as f64 / 2.0;
    let mut blocks = data.chunks(block_size).peekable();

    let dists = (0..(block_count / 2))
        .map(|_| normalised_hamming_distance(blocks.next().unwrap().iter(), blocks.peek().unwrap().iter(), block_size));

    dists.sum::<f64>() / distance_count
}

fn transpose(blocks: &Vec<&[u8]>, block_size: usize) -> Vec<Vec<u8>> {
    let mut transposed_blocks = vec![Vec::<u8>::new(); block_size];

    for i in 0..blocks.len() {
        for j in 0..block_size {
            match blocks[i].get(j) {
                Some(&x) => transposed_blocks[j].push(x),
                None => (),
            }
        }
    }

    transposed_blocks
}

pub fn decrypt_xor_cipher(data: &Vec<u8>) -> (Vec<u8>, Vec<u8>) {
    let weights = get_weights();

    let (key_size, _) = (2..40)
        .map(|key_len| (key_len, n_block_average_dist(data, 20, key_len))) // why is such a high value necessary
        .min_by(|(_, dist1), (_, dist2)| dist1.partial_cmp(dist2).unwrap())
        .unwrap();

    let blocks = data.chunks(key_size).collect::<Vec<_>>();
    let blocks = transpose(&blocks, key_size);
    let full_key = blocks
        .iter()
        .map(|data| guess_xor_bytes(&data, &weights).unwrap().key)
        .collect::<Vec<_>>();
    let data = encode(data, &full_key);
    (full_key, data)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hamming_distance() {
        assert_eq!(hamming_distance(b"this is a test".iter(), b"wokka wokka!!!".iter()), 37);
    }

    #[test]
    fn test_normalised_hamming_distance() {
        assert_approx_eq!(
            normalised_hamming_distance(b"this is a test".iter(), b"wokka wokka!!!".iter(), 10),
            2.5
        );
    }

    #[test]
    fn test_n_block_average_dist() {
        let data = (0..100).collect();

        assert_approx_eq!(
            n_block_average_dist(&data, 2, 5),
            normalised_hamming_distance(data.iter(), data.iter().skip(5), 5)
        );

        assert_approx_eq!(
            n_block_average_dist(&data, 4, 5),
            (normalised_hamming_distance(data.iter(), data.iter().skip(5), 5)
                + normalised_hamming_distance(data.iter().skip(5), data.iter().skip(10), 5))
                / 2.0
        );
    }

    #[test]
    fn test_transpose() {
        let input = (1u8..12u8).collect::<Vec<_>>();
        let input = input.chunks(3).collect::<Vec<_>>();

        assert_eq!(
            transpose(&input, 3),
            vec![vec![1, 4, 7, 10], vec![2, 5, 8, 11], vec![3, 6, 9],]
        );
    }
}
