use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map: HashMap<[u8; 26], Vec<String>> = HashMap::new();

    for str in strs {
        let mut key = [0_u8; 26];
        
        for c in str.chars() {
            key[c as usize - 'a' as usize ] += 1;
        }

        if let Some(value) = map.get_mut(&key) {
            value.push(str)
        } else {
            map.insert(key, vec![str]);
        }
    }

    map.into_values().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    //TODO Flaky
    #[test]
    fn test_case_1() {
        let input = vec!["eat".to_string(),"tea".to_string(),"tan".to_string(),"ate".to_string(),"nat".to_string(),"bat".to_string()];
        let output = vec![vec!["bat".to_string()],vec!["eat".to_string(),"tea".to_string(),"ate".to_string()],vec!["tan".to_string(),"nat".to_string()]];
        assert_eq!(output, group_anagrams(input));
    }

    #[test]
    fn test_case_2() {
        let input = vec!("".to_string());
        let output = vec!(vec!("".to_string())); 
        assert_eq!(output, group_anagrams(input));
    }

    #[test]
    fn test_case_3() {
        let input = vec!("a".to_string());
        let output = vec!(vec!("a".to_string())); 
        assert_eq!(output, group_anagrams(input));
    }
}