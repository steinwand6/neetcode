use std::collections::HashSet;

use crate::Solution;

impl Solution {
    // time: O(n)
    // space: O(n)
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut hashset = HashSet::new();
        !nums.iter().all(|n| hashset.insert(n))
    }
    #[allow(dead_code)]
    // time: O(nlogn)
    // space: O(1)
    fn contains_duplicate_old2(nums: Vec<i32>) -> bool {
        let mut nums = nums.clone();
        nums.sort();
        for (idx, x) in nums.iter().enumerate() {
            if idx != 0 {
                if let Some(prev) = nums.get(idx - 1) {
                    if x == prev {
                        return true;
                    }
                }
            }
            if let Some(next) = nums.get(idx + 1) {
                if x == next {
                    return true;
                }
            }
        }
        false
    }
    #[allow(dead_code)]
    // time limit exceeded
    // time: O(n^2)
    // space: O(1)
    fn contains_duplicate_old1(nums: Vec<i32>) -> bool {
        for (idx, x) in nums.iter().enumerate() {
            for j in nums[(idx + 1)..].iter() {
                if x == j {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;
    #[test]
    fn test1() {
        let nums = vec![1, 2, 3, 1];
        assert!(Solution::contains_duplicate(nums));
    }
    #[test]
    fn test2() {
        let nums = vec![1, 2, 3, 4];
        assert!(!Solution::contains_duplicate(nums));
    }
    #[test]
    fn test3() {
        let nums = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
        assert!(Solution::contains_duplicate(nums));
    }
}
