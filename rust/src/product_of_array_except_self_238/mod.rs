pub fn product_except_self_slow(nums: Vec<i32>) -> Vec<i32> {
    let mut output = vec![1; nums.len()];
    for (i, n) in nums.iter().enumerate() {
        for (im, m) in output.iter_mut().enumerate() {
            if i != im {
                *m *= n;
            }
        }
    }
    output
}

pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut output = vec![1; nums.len()];
    nums.iter().enumerate().fold(1, | v, (i, n)| {
        output[i] = v * output[i];
        v * *n
    });
    nums.iter().enumerate().rev().fold(1, | v, (i, n)| {
        output[i] = v * output[i];
        v * *n
    });
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(vec!(24,12,8,6), product_except_self(vec!(1,2,3,4)));
    }

    #[test]
    fn test_case_2() {
        assert_eq!(vec!(0,0,9,0,0), product_except_self(vec!(-1,1,0,-3,3)));
    }
}