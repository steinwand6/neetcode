use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn two_sum2(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut head = 0;
        let mut tail = numbers.len() - 1;
        loop {
            let tmp = numbers[head] + numbers[tail];
            if tmp == target {
                return vec![head as i32 + 1, tail as i32 + 1];
            } else if tmp > target {
                tail -= 1;
            } else {
                head += 1;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test1() {
        let numbers = vec![2, 7, 11, 15];
        let target = 9;
        let expect = vec![1, 2];
        assert_eq!(Solution::two_sum2(numbers, target), expect);
    }
}
