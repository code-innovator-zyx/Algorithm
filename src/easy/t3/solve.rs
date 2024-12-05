pub fn is_palindrome(s: String) -> bool {
    let s: Vec<char> = s.chars().collect();
    let mut left: i32 = 0;
    let mut right: i32 = s.len() as i32 - 1;
    while left < right {
        loop {
            if left >= right {
                break;
            }
            if s[left as usize].is_ascii_alphanumeric() {
                break;
            }
            left += 1;
        }
        loop {
            if left >= right {
                break;
            }

            if s[right as usize].is_ascii_alphanumeric() {
                break;
            }
            right -= 1;
        }
        if s[left as usize].to_lowercase().to_string()
            != s[right as usize].to_lowercase().to_string()
        {
            return false;
        }
        left += 1;
        right -= 1;
    }

    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        let s = String::from("0P");
        let result = is_palindrome(s);
        assert_eq!(result, false)
    }
}
