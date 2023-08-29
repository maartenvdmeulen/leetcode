pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack = Vec::<i32>::new();
    tokens.into_iter().for_each(|x| {
        if let Ok(y) = x.parse::<i32>() {
            stack.push(y)
        } else {
            let arg2 = stack.pop().unwrap();
            let arg1 = stack.pop().unwrap();
            match x.as_str() {
                "+" => stack.push(arg1 + arg2),
                "-" => stack.push(arg1 - arg2),
                "*" => stack.push(arg1 * arg2),
                "/" => stack.push(arg1 / arg2),
                _ => panic!()
            }
        }
    });

    stack.pop().unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(9, eval_rpn(vec!["2".to_string(), "1".to_string(), "+".to_string(), "3".to_string(), "*".to_string()]));
    }

    #[test]
    fn test_case_2() {
        assert_eq!(6, eval_rpn(vec!["4".to_string(), "13".to_string(), "5".to_string(), "/".to_string(), "+".to_string()]));
    }

    #[test]
    fn test_case_3() {
        assert_eq!(22, eval_rpn(vec!["10".to_string(), "6".to_string(), "9".to_string(), "3".to_string(), "+".to_string(), "-11".to_string(), "*".to_string(), "/".to_string(), "*".to_string(), "17".to_string(), "+".to_string(), "5".to_string(), "+".to_string()]));
    }
}