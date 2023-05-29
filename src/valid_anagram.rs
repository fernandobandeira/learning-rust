use std::iter::Iterator;

fn valid_anagram_simple(s: String, t: String) -> bool {
    // If the strings have different lengths, they can't be anagrams.
    if s.len() != t.len() {
        return false;
    }

    // First, we convert the strings into vectors of characters.
    let mut s_iter: Vec<char> = s.chars().collect();
    let mut t_iter: Vec<char> = t.chars().collect();

    // We sort the vectors in-place, so we don't have to allocate a new one.
    s_iter.sort_unstable();
    t_iter.sort_unstable();

    // If the sorted vectors are equal, it means the strings are anagrams.
    return s_iter == t_iter;
}

const ALPHABET_SIZE: usize = 26;
const ASCII_OFFSET: usize = 97;
fn valid_anagram(s: String, t: String) -> bool {
    // If the strings have different lengths, they can't be anagrams.
    if s.len() != t.len() {
        return false;
    }

    // We convert the strings into byte arrays.
    let s_bytes = s.as_bytes();
    let t_bytes = t.as_bytes();

    // This problem only deals with lowercase letters, so we can use a fixed-size array.
    let mut char_counts = [0; ALPHABET_SIZE];

    for index in 0..s_bytes.len() {
        // We subtract the ASCII offset to get the index of the character in the array.
        // For example, 'a' - 'a' = 0, 'b' - 'a' = 1, 'c' - 'a' = 2, etc.

        // We then increment the count for that character for the first string.
        char_counts[s_bytes[index] as usize - ASCII_OFFSET] += 1;

        // Finally, we decrement the count for that character for the second string.
        char_counts[t_bytes[index] as usize - ASCII_OFFSET] -= 1;
    }

    // If the strings are anagrams, all the counts should be zero.
    return char_counts.iter().all(|&count| count == 0);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_examples() {
        let inputs = vec![
            vec!["anagram", "nagaram"],
            vec!["rat", "car"],
            vec!["foo", "fooo"],
            vec!["foo", "off"],
        ];
        let expected = vec![true, false, false, false];

        // zip() will create a tuple of each element in the two vectors.
        for (input, expected) in inputs.into_iter().zip(expected) {
            assert_eq!(
                valid_anagram(String::from(input[0]), String::from(input[1])),
                expected
            );
        }
    }

    #[bench]
    fn bench_simple_contains(b: &mut Bencher) {
        b.iter(|| valid_anagram_simple(String::from("anagram"), String::from("nagaram")));
    }

    #[bench]
    fn bench_optimized_contains(b: &mut Bencher) {
        b.iter(|| valid_anagram(String::from("anagram"), String::from("nagaram")));
    }
}
