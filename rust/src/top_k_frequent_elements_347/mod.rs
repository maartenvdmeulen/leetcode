use std::collections::HashMap;

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();

    for num in nums.into_iter() {
        let amount = map.entry(num).or_insert(0);
        *amount += 1; 
    }

    let mut z: Vec<(i32, i32)> = map.into_iter().collect();
    z.sort_by_key(|&(_, y)| y);
    z.reverse();
    z.iter().map(|&(x, _)| x).take(k as usize).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(vec![1, 2], top_k_frequent(vec![1,1,1,2,2,3], 2));
    }

    #[test]
    fn test_case_2() {
        assert_eq!(vec![1], top_k_frequent(vec![1], 1));
    }
}