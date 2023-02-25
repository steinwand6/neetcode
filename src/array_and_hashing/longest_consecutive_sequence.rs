use crate::Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        //use std::iter::FromIterator; // compile error on leetcode without this
        let set: HashSet<i32> = HashSet::from_iter(nums.into_iter());
        let mut ans = 0;
        for &n in set.iter() {
            let mut indicate = n;
            let mut tmp = 1;
            if !set.contains(&(indicate + 1)) {
                while set.contains(&(indicate - 1)) {
                    tmp += 1;
                    indicate -= 1;
                }
            }
            if tmp > ans {
                ans = tmp;
            }
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test1() {
        let nums = vec![100, 4, 200, 1, 3, 2];
        let expect = 4;
        assert_eq!(Solution::longest_consecutive(nums), expect);
    }

    #[test]
    fn test2() {
        let nums = vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1];
        let expect = 9;
        assert_eq!(Solution::longest_consecutive(nums), expect);
    }
}
