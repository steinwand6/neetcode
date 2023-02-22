use crate::Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut sort_s: Vec<char> = s.chars().collect();
        sort_s.sort();
        let mut sort_t: Vec<char> = t.chars().collect();
        sort_t.sort();
        for i in 0..s.len() {
            if sort_s[i] != sort_t[i] {
                return false;
            }
        }
        return true;
    }
}

#[cfg(test)]
mod valid_anagram_test {
    use crate::Solution;

    #[test]
    fn test1() {
        let s = "anagram".to_string();
        let t = "nagaram".to_string();
        assert!(Solution::is_anagram(s, t));
    }
    #[test]
    fn test2() {
        let s = "rat".to_string();
        let t = "car".to_string();
        assert!(!Solution::is_anagram(s, t));
    }
}
