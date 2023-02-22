use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut hashmap = HashMap::new();
        for n in nums.into_iter() {
            let counter = hashmap.entry(n).or_insert(0);
            *counter += 1;
        }
        let mut vec: Vec<(i32, i32)> = hashmap.into_iter().collect();
        vec.sort_by(|a, b| (a.1).cmp(&(b.1)));
        let mut answer = vec![];
        for _ in 0..k {
            answer.push(vec.pop().unwrap().0);
        }
        answer
    }
}

#[cfg(test)]
mod top_k_frequent_element {
    use crate::Solution;

    #[test]
    fn test1() {
        let nums = vec![1, 1, 1, 2, 2, 3];
        let k = 2;
        assert_eq!(Solution::top_k_frequent(nums, k), vec![1, 2])
    }
}
