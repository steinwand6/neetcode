use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hashmap: HashMap<i32, usize> = HashMap::with_capacity(nums.len());

        for (idx, num) in nums.into_iter().enumerate() {
            if let Some(ans_1) = hashmap.get(&(target - num)) {
                return vec![*ans_1 as i32, idx as i32];
            }
            hashmap.insert(num, idx);
        }
        unreachable!()
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let expect = vec![0, 1];
        assert_eq!(Solution::two_sum(nums, target), expect);
    }

    #[test]
    fn test2() {
        let nums = vec![3, 2, 4];
        let target = 6;
        let expect = vec![1, 2];
        assert_eq!(Solution::two_sum(nums, target), expect);
    }

    #[test]
    fn test3() {
        let nums = vec![3, 3];
        let target = 6;
        let expect = vec![0, 1];
        assert_eq!(Solution::two_sum(nums, target), expect);
    }
}
