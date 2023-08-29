use std::collections::HashMap;

pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }
    
    let mut set = HashMap::<char, i32>::with_capacity(s.len());

    s.chars().for_each(|c| *set.entry(c).or_default() += 1);
    t.chars().for_each(|c| *set.entry(c).or_default() -= 1);
    
    set.into_values().all(|amount| amount == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(true, is_anagram("anagram".to_string(), "nagaram".to_string()));
    }

    #[test]
    fn test_case_2() {
        assert_eq!(false, is_anagram("rat".to_string(), "car".to_string()));
    }
}