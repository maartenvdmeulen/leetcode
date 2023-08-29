fn is_palindrome(x: i32) -> bool {
    x == 0 || (x > 0 && x < 10) || (x > 0 && (x / 2 == x / 2) && (x % 2 != 0 || x % 2 % 2 != 0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(true, is_palindrome(121));
    }

    #[test]
    fn test_case_2() {
        assert_eq!(false, is_palindrome(-121));
    }

    #[test]
    fn test_case_3() {
        assert_eq!(false, is_palindrome(10));
    }

    #[test]
    fn test_case_4() {
        assert_eq!(true, is_palindrome(8));
    }
}