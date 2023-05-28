use std::collections::HashSet;

fn contains_duplicate_simple(nums: Vec<i32>) -> bool {
    // Sets are structures that can only contain unique values.
    // Here we use the with_capacity() method to pre-allocate the memory for the set.
    let mut set = HashSet::with_capacity(nums.len());

    for num in nums {
        // If the set already contains the value, it'll fail to insert it.
        if !set.insert(num) {
            return true;
        }
    }

    // If we reach this point, it means the set doesn't contain any duplicates.
    return false;
}

fn contains_duplicate(mut nums: Vec<i32>) -> bool {
    // We sort the vector in-place, so we don't have to allocate a new one.
    nums.sort_unstable();

    // We use the windows() method to create an iterator that returns a slice of the vector.
    // The slice will have a length of 2, and will move one element at a time.
    // For example, if we have a vector of [1, 2, 3, 4], the iterator will return:
    // [1, 2], [2, 3], [3, 4]
    // We then use the any() method to check if any of the slices contain the same value.
    // If they do, it'll return true, otherwise it'll return false.
    nums.windows(2).any(|window| window[0] == window[1])
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_examples() {
        let inputs = vec![
            vec![1, 2, 3, 1],
            vec![1, 2, 3, 4],
            vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2],
        ];
        let expected = vec![true, false, true];

        // zip() will create a tuple of each element in the two vectors.
        for (input, expected) in inputs.into_iter().zip(expected) {
            assert_eq!(contains_duplicate(input), expected);
        }
    }

    #[bench]
    fn bench_simple_contains(b: &mut Bencher) {
        b.iter(|| contains_duplicate_simple(vec![1, 2, 3, 1]));
    }

    #[bench]
    fn bench_optimized_contains(b: &mut Bencher) {
        b.iter(|| contains_duplicate(vec![1, 2, 3, 1]));
    }
}
