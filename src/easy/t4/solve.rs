pub fn is_subsequence(s: String, t: String) -> bool {
    let s: Vec<char> = s.chars().collect();
    let t: Vec<char> = t.chars().collect();
    let mut ps: usize = 0;
    t.iter().for_each(|str| {
        if ps == s.len() {
            return;
        }
        if s[ps] == *str {
            ps += 1
        }
    });
    if ps != s.len() {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_subsequence() {
        let s = String::from("abc");
        let t = String::from("ahbgdc");
        let result = is_subsequence(s, t);
        assert_eq!(result, true)
    }

    #[test]
    fn test_iter() {
        (0..5).map(|x| x + 1).for_each(|x| println!("{x}"));
    }
}
