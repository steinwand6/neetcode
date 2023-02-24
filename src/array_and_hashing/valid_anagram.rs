use std::collections::HashMap;

use crate::Solution;

impl Solution {
    // time: O(n)
    // space: O(n)
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut map_s = HashMap::new();
        let mut map_t = HashMap::new();
        let s = s.as_bytes();
        let t = t.as_bytes();
        for i in 0..s.len() {
            map_s.entry(s[i]).and_modify(|c| *c += 1).or_insert(0);
            map_t.entry(t[i]).and_modify(|c| *c += 1).or_insert(0);
        }
        map_s == map_t
    }
    #[allow(dead_code)]
    // time: O(nlogn)
    // space: O(n)
    fn is_anagram_sol1(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut sort_s: Vec<char> = s.chars().collect();
        sort_s.sort();
        let mut sort_t: Vec<char> = t.chars().collect();
        sort_t.sort();
        sort_s == sort_t
    }
}

#[cfg(test)]
mod test {
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
