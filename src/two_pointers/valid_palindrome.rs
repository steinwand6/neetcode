use crate::Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s_rev = s
            .chars()
            .rev()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_lowercase());
        s.chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .zip(s_rev)
            .all(|(c1, c2)| c1 == c2)
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test1() {
        let s = "A man, a plan, a canal: Panama".to_string();
        assert!(Solution::is_palindrome(s));
    }
    #[test]
    fn test2() {
        let s = "race a car".to_string();
        assert!(!Solution::is_palindrome(s));
    }
    #[test]
    fn test3() {
        let s = " ".to_string();
        assert!(Solution::is_palindrome(s));
    }
    #[test]
    fn test4() {
        let s = "0P".to_string();
        assert!(!Solution::is_palindrome(s));
    }
}
