use std::iter::Iterator;

fn valid_anagram(s: String, t: String) -> bool {
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
            vec!["foo", "off"]
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
        b.iter(|| valid_anagram(String::from("anagram"), String::from("nagaram")));
    }
}
