use crate::Solution;
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::cmp::Ordering;
        nums.sort();
        let mut res = Vec::new();
        for i in 0..nums.len() - 1 {
            if i != 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let (mut l, mut r) = (i + 1, nums.len() - 1);
            while l < r {
                match (nums[i] + nums[l] + nums[r]).cmp(&0) {
                    Ordering::Less => l += 1,
                    Ordering::Greater => r -= 1,
                    Ordering::Equal => {
                        res.push(vec![nums[i], nums[l], nums[r]]);
                        l += 1;
                        while l < r && nums[l] == nums[l - 1] {
                            l += 1;
                        }
                    }
                }
            }
        }
        res
    }

    // time limit exceeded
    pub fn three_sum_old(nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::collections::HashSet;
        let mut res = HashSet::with_capacity(nums.len());
        for (i, &num1) in nums.iter().enumerate() {
            let mut exist_nums = HashSet::with_capacity(nums.len());
            let mut j = i + 1;
            for &num2 in &nums.as_slice()[i + 1..] {
                for &num in nums[j + 1..].iter() {
                    exist_nums.insert(num as i32);
                }
                if let Some(&num3) = exist_nums.get(&-(num1 + num2)) {
                    let mut triples = vec![num1, num2, num3];
                    triples.sort();
                    res.insert(triples);
                }
                exist_nums.clear();
                j += 1;
            }
        }
        res.iter().map(|v| v.to_vec()).collect()
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test1() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let expect: Vec<Vec<i32>> = [[-1, -1, 2], [-1, 0, 1]]
            .iter()
            .map(|a| a.to_vec())
            .collect();
        assert_eq!(Solution::three_sum(nums), expect);
    }

    #[test]
    fn test2() {
        let nums = vec![1, 2, -2, -1];
        let expect: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::three_sum(nums), expect);
    }

    #[test]
    fn test3() {
        let nums = vec![0, 0, 0, 0, 0, 0];
        let expect: Vec<Vec<i32>> = vec![vec![0, 0, 0]];
        assert_eq!(Solution::three_sum(nums), expect);
    }
}
