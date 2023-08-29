use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut values = HashSet::with_capacity(nums.len());

    for n in nums.iter() {
        if !values.insert(n) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(true, contains_duplicate(vec![1,2,3,1]));
    }

    #[test]
    fn test_case_2() {
        assert_eq!(false, contains_duplicate(vec![1,2,3,4]));
    }

    #[test]
    fn test_case_3() {
        assert_eq!(true, contains_duplicate(vec![1,1,1,3,3,4,3,2,4,2]));
    }
}