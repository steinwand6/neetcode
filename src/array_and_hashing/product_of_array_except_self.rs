use crate::Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![1; nums.len()];
        for i in (0..(nums.len() - 1)).rev() {
            ans[i] = ans[i + 1] * nums[i + 1];
        }
        let mut pre = 1;
        for i in 0..nums.len() {
            ans[i] = ans[i] * pre;
            pre = pre * nums[i];
        }
        ans
    }

    #[allow(dead_code)]
    fn product_except_self_old2(nums: Vec<i32>) -> Vec<i32> {
        let mut prefix = vec![1; nums.len()];
        let mut postfix = vec![1; nums.len()];
        for i in 1..nums.len() {
            prefix[i] = prefix[i - 1] * nums[i - 1];
        }
        for i in (0..(nums.len() - 1)).rev() {
            postfix[i] = postfix[i + 1] * nums[i + 1];
        }
        let mut ans = vec![1; nums.len()];
        for i in 0..nums.len() {
            ans[i] = prefix[i] * postfix[i];
        }
        ans
    }

    #[allow(unused)]
    fn product_except_self_old1(nums: Vec<i32>) -> Vec<i32> {
        let count_zero = nums.iter().filter(|&n| *n == 0).count();
        if count_zero == 0 {
            let product_all = nums.iter().fold(1, |acc, n| acc * n);
            nums.into_iter().map(|n| product_all / n).collect()
        } else if count_zero > 1 {
            nums.iter().map(|_| 0).collect()
        } else {
            let product_without_zero = nums.iter().filter(|n| **n != 0).fold(1, |acc, n| *n * acc);
            nums.into_iter()
                .map(|n| if n != 0 { 0 } else { product_without_zero })
                .collect()
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test1() {
        let nums = vec![1, 2, 3, 4];
        let expect = vec![24, 12, 8, 6];
        assert_eq!(Solution::product_except_self(nums), expect);
    }

    #[test]
    fn test2() {
        let nums = vec![-1, 1, 0, -3, 3];
        let expect = vec![0, 0, 9, 0, 0];
        assert_eq!(Solution::product_except_self(nums), expect);
    }

    #[test]
    fn test3() {
        let nums = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
        let expect = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(Solution::product_except_self(nums), expect);
    }
}
