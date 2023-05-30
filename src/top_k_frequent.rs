use std::collections::HashMap;

fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    // We store the numbers and their frequencies in a HashMap.
    let mut map: HashMap<i32, i32> = HashMap::with_capacity(nums.len());

    for num in nums {
        // We increment the frequency of the number.
        map.entry(num).and_modify(|c: &mut i32| *c += 1).or_insert(1);
    }

    // We sort the numbers by their frequencies.
    let mut sorted: Vec<(&i32, &i32)> = map.iter().collect();
    sorted.sort_by(|a: &(&i32, &i32), b: &(&i32, &i32)| b.1.cmp(a.1));

    // We return the first k numbers.
    return sorted.iter().take(k as usize).map(|(num, _)| **num).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_examples() {
        let inputs: Vec<(Vec<i32>, i32)> = vec![(vec![1,1,1,2,2,3], 2), (vec![1], 1)];
        let expected: Vec<Vec<i32>> = vec![vec![1, 2], vec![1]];

        for (input, expected) in inputs.into_iter().zip(expected) {
            assert_eq!(top_k_frequent(input.0, input.1), expected);
        }
    }

    #[bench]
    fn bench_top_k_frequent(b: &mut Bencher) {
        b.iter(|| top_k_frequent(vec![1,1,1,2,2,3], 2));
    }
}