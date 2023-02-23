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
        for i in 0..s.len() {
            let counter_s = map_s.entry(s.as_bytes()[i]).or_insert(0);
            let counter_t = map_t.entry(t.as_bytes()[i]).or_insert(0);
            *counter_s += 1;
            *counter_t += 1;
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
