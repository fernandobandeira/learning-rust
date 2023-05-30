fn product_except_self_simple(nums: Vec<i32>) -> Vec<i32> {
    let nums_len = nums.len();

    // left[i] and right[i] contain product of all elements to the left and right of index i
    let mut left: Vec<i32> = vec![1; nums_len];
    let mut right: Vec<i32> = vec![1; nums_len];

    for i in 1..nums_len {
        left[i] = left[i - 1] * nums[i - 1];
        right[nums_len - i - 1] = right[nums_len - i] * nums[nums_len - i];
    }
    
    // Since we are not allowed to use division, we can just multiply left and right
    let mut result: Vec<i32> = vec![1; nums_len];
    for i in 0..nums_len {
        result[i] = left[i] * right[i];
    }

    return result;
}

fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let nums_len = nums.len();

    let mut result: Vec<i32> = vec![1; nums_len];

    // We'll populate the result array with the product of all elements to the left of index i
    let mut left_acc: i32 = 1;
    for i in 0..nums_len {
        result[i] = left_acc;
        left_acc *= nums[i];
    }
    
    // We'll simply repeat the process, but this time we'll multiply the result with the product of all elements to the right of index i
    let mut right_acc: i32 = 1;
    for i in (0..nums_len).rev() {
        result[i] *= right_acc;
        right_acc *= nums[i];
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_examples() {
        let inputs: Vec<Vec<i32>> = vec![vec![1, 2, 3, 4], vec![-1, 1, 0, -3, 3]];
        let expected: Vec<Vec<i32>> = vec![vec![24, 12, 8, 6], vec![0, 0, 9, 0, 0]];

        for (input, expected) in inputs.into_iter().zip(expected) {
            assert_eq!(product_except_self(input), expected);
        }
    }

    #[bench]
    fn bench_product_except_self_simple(b: &mut Bencher) {
        b.iter(|| product_except_self_simple(vec![1, 2, 3, 4]));
    }

    #[bench]
    fn bench_product_except_self(b: &mut Bencher) {
        b.iter(|| product_except_self(vec![1, 2, 3, 4]));
    }
}
