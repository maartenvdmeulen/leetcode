fn two_sum_slow(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in i+1..nums.len() {
            if nums[i] + nums[j] == target {
                return vec![i as i32, j as i32]
            }
        }
    }
    unreachable!()
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = std::collections::HashMap::<&i32, i32>::with_capacity(nums.len() - 1);

    for (i, x) in nums.iter().enumerate() {
        if let Some(j) = map.get(&(target - x)) {
            return vec![*j, i as i32];
        }
        map.insert(x, i as i32);
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        assert_eq!(vec![0, 1], two_sum(nums, target));
    }
}
