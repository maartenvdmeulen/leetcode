pub fn is_palindrome(s: String) -> bool {
    if s.len() == 0 {
        return true;
    }
    
    let chars: Vec<char> = s.chars()
        .filter(|c| c.is_ascii_alphabetic() || c.is_ascii_digit())
        .map(|c| c.to_ascii_lowercase())
        .collect();

    if chars.len() == 0 || chars.len() == 1 {
        return true;
    }

    let (mut start_index, mut end_index) = (0, chars.len() - 1);
    loop {
        if chars[start_index] != chars[end_index] {
            return false;
        }
        if start_index == end_index || start_index + 1 == end_index {
            break;
        } else {
            start_index += 1;
            end_index -= 1;
        }
    } 

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(true, is_palindrome("A man, a plan, a canal: Panama".to_string()));
    }

    #[test]
    fn test_case_2() {
        assert_eq!(false, is_palindrome("race a car".to_string()));
    }

    #[test]
    fn test_case_3() {
        assert_eq!(true, is_palindrome(" ".to_string()));
    }

    #[test]
    fn test_case_4() {
        assert_eq!(false, is_palindrome("0P".to_string()));
    }
}