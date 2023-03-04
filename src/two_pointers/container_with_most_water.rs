use crate::Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max = 0;
        let (mut l, mut r) = (0, height.len() - 1);
        while l < r {
            let lower = height[l].min(height[r]) as usize;
            max = max.max(lower * (r - l));
            if height[l] < height[r] {
                l += 1;
            } else {
                r -= 1;
            }
        }
        max as i32
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test1() {
        let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        let expect = 49;
        assert_eq!(Solution::max_area(height), expect);
    }

    #[test]
    fn test2() {
        let height = vec![1, 1];
        let expect = 1;
        assert_eq!(Solution::max_area(height), expect);
    }
}
