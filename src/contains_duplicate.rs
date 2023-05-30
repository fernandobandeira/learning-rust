use std::collections::HashSet;

fn contains_duplicate_simple(nums: Vec<i32>) -> bool {
    let mut set: HashSet<i32> = HashSet::with_capacity(nums.len());

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
    // For example, if we have a vector of [1, 2, 3, 4], the iterator will return:
    // [1, 2], [2, 3], [3, 4]
    // We then check if any of the slices contain two equal numbers.
    return nums.windows(2).any(|window: &[i32]| window[0] == window[1]);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_examples() {
        let inputs: Vec<Vec<i32>> = vec![
            vec![1, 2, 3, 1],
            vec![1, 2, 3, 4],
            vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2],
        ];
        let expected: Vec<bool> = vec![true, false, true];

        for (input, expected) in inputs.into_iter().zip(expected) {
            assert_eq!(contains_duplicate(input), expected);
        }
    }

    #[bench]
    fn bench_contains_duplicate_simple(b: &mut Bencher) {
        b.iter(|| contains_duplicate_simple(vec![1, 2, 3, 1]));
    }

    #[bench]
    fn bench_contains_duplicate(b: &mut Bencher) {
        b.iter(|| contains_duplicate(vec![1, 2, 3, 1]));
    }
}
