use std::collections::VecDeque;

pub fn is_valid(s: String) -> bool {
    let mut stack = VecDeque::<char>::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '(' | '{' | '[' => {
                stack.push_back(c)
            },
            ')' => {
                if stack.len() == 0 || stack.pop_back().unwrap() != '(' {
                    return false;
                }
            },
            '}' => {
                if stack.len() == 0 || stack.pop_back().unwrap() != '{' {
                    return false;
                }
            },
            ']' => {
                if stack.len() == 0 || stack.pop_back().unwrap() != '[' {
                    return false;
                }
            },
            _ => panic!()
        }  
    }
    stack.len() == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(true, is_valid("()".to_string()));
    }

    #[test]
    fn test_case_2() {
        assert_eq!(true, is_valid("()[]{}".to_string()));
    }

    #[test]
    fn test_case_3() {
        assert_eq!(false, is_valid("(]".to_string()));
    }

    #[test]
    fn test_case_4() {
        assert_eq!(false, is_valid("([)]".to_string()));
    }
}