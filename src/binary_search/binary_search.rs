use crate::Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        use std::cmp::Ordering;
        let (mut left, mut right) = (0, nums.len() - 1);

        let mut overflow = false;
        while !overflow && right >= left {
            let mid = (left + right) / 2;
            match target.cmp(&nums[mid]) {
                Ordering::Equal => {
                    return mid as i32;
                }
                Ordering::Greater => (left, overflow) = mid.overflowing_add(1),
                Ordering::Less => (right, overflow) = mid.overflowing_sub(1),
            }
        }
        -1
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test1() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 9;
        let expect = 4;
        assert_eq!(Solution::search(nums, target), expect);
    }
    #[test]
    fn test2() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 2;
        let expect = -1;
        assert_eq!(Solution::search(nums, target), expect);
    }
    #[test]
    fn test3() {
        let nums = vec![5];
        let target = -5;
        let expect = -1;
        assert_eq!(Solution::search(nums, target), expect);
    }
    #[test]
    fn test4() {
        let nums = vec![-1, 0, 3];
        let target = -1;
        let expect = 0;
        assert_eq!(Solution::search(nums, target), expect);
    }
}
