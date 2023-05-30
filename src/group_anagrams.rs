use std::collections::HashMap;

fn group_anagrams_simple(strs: Vec<String>) -> Vec<Vec<String>> {
    // The key will be the sorted string, and the value will be a vector of the original strings.
    let mut map: HashMap<Vec<char>, Vec<String>> = HashMap::with_capacity(strs.len());

    for s in strs {
        let mut chars: Vec<char> = s.chars().collect();
        chars.sort_unstable();

        // If the key already exists, we append the string to the vector.
        // If the key doesn't exist, we create a new vector with the string.
        map.entry(chars).or_insert(vec![]).push(s);
    }

    // We convert the HashMap into a vector of vectors.
    return map.into_iter().map(|(_, v)| v).collect();
}

const ALPHABET_SIZE: usize = 26;
const ASCII_OFFSET: usize = 'a' as usize;
fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    // The key will be a fixed-size array of character counts, and the value will be a vector of the original strings.
    let mut map: HashMap<[u8; ALPHABET_SIZE], Vec<String>> = HashMap::with_capacity(strs.len());

    for s in strs.into_iter() {
        // This problem only deals with lowercase letters, so we can use a fixed-size array.
        let mut chars: [u8; ALPHABET_SIZE] = [0; ALPHABET_SIZE];

        for char in s.chars() {
            // We subtract the ASCII offset to get the index of the character in the array.
            // For example, 'a' - 'a' = 0, 'b' - 'a' = 1, 'c' - 'a' = 2, etc.
            chars[char as usize - ASCII_OFFSET] += 1;
        }

        // If the key already exists, we append the string to the vector.
        // If the key doesn't exist, we create a new vector with the string.
        map.entry(chars).or_insert(vec![]).push(s);
    }

    // We convert the HashMap into a vector of vectors.
    let mut arr: Vec<Vec<String>> = Vec::new();

    for v in map.into_values() {
        arr.push(v);
    }

    return arr;
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_examples() {
        let inputs: Vec<Vec<String>> = vec![
            vec_of_strings!["eat", "tea", "tan", "ate", "nat", "bat"],
            vec_of_strings![""],
            vec_of_strings!["a"],
        ];
        let expected: Vec<Vec<Vec<String>>> = vec![
            vec![
                vec_of_strings!["eat", "tea", "ate"],
                vec_of_strings!["tan", "nat"],
                vec_of_strings!["bat"],
            ],
            vec![vec_of_strings![""]],
            vec![vec_of_strings!["a"]],
        ];

        for (input, expected) in inputs.into_iter().zip(expected) {
            assert!(group_anagrams(input).iter().all(|v| expected.contains(v)));
        }
    }

    #[bench]
    fn bench_group_anagrams_simple(b: &mut Bencher) {
        b.iter(|| group_anagrams_simple(vec_of_strings!["eat", "tea", "tan", "ate", "nat", "bat"]));
    }

    #[bench]
    fn bench_group_anagrams(b: &mut Bencher) {
        b.iter(|| group_anagrams(vec_of_strings!["eat", "tea", "tan", "ate", "nat", "bat"]));
    }
}
