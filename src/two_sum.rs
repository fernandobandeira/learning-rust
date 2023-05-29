use std::collections::HashMap;

fn two_sum_simple(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // We create a HashMap to store the values we've seen so far.
    // The key will be the number, and the value will be the index.
    let mut map: HashMap<i32, usize> = HashMap::with_capacity(nums.len());

    // We iterate over the vector.
    for (index, &num_ptr) in nums.iter().enumerate() {
        // We calculate the complement of the current number.
        // The complement is the number we need to add to the current number to get the target.
        let complement = target - num_ptr;

        // We check if the complement is in the HashMap.
        // If it is, it means we've already seen it, and we can return the solution.
        if map.contains_key(&complement) {
            return vec![map.get(&complement).copied().unwrap() as i32, index as i32];
        }

        // If we haven't seen the complement yet, we add the current number to the HashMap.
        map.insert(nums[index], index);
    }

    return vec![];
}

// Instead of storing the numbers that we have seen so far, we store the numbers we're looking for.
// This way we can avoid some unnecessary allocations.
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // We create a HashMap to store the values we're looking for.
    // The key will be the number, and the value will be the index.
    let mut map: HashMap<i32, usize> = HashMap::with_capacity(nums.len());

    // We iterate over the vector.
    for (index, &num_ptr) in nums.iter().enumerate() {
        // We check if the current number is in the HashMap.
        // If it is, it means we're looking for it, and we can return the solution.
        if map.contains_key(&num_ptr) {
            return vec![map.get(&num_ptr).copied().unwrap() as i32, index as i32];
        }

        // Otherwise, we store the complement of the current number in the HashMap.
        map.insert(target - num_ptr, index);
    }

    return vec![];
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_examples() {
        let inputs = vec![
            (vec![2, 7, 11, 15], 9),
            (vec![3, 2, 4], 6),
            (vec![3, 3], 6),
        ];
        let expected = vec![
            vec![0,1],
            vec![1,2],
            vec![0,1],
        ];

        // zip() will create a tuple of each element in the two vectors.
        for (input, expected) in inputs.into_iter().zip(expected) {
            assert_eq!(
                two_sum(input.0, input.1),
                expected
            );
        }
    }

    #[bench]
    fn bench_two_sum_simple(b: &mut Bencher) {
        b.iter(|| two_sum_simple(vec![2, 7, 11, 15], 9));
    }

    #[bench]
    fn bench_two_sum_optimized(b: &mut Bencher) {
        b.iter(|| two_sum(vec![2, 7, 11, 15], 9));
    }
}