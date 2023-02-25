use crate::Solution;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0_i32; temperatures.len()];
        let mut stack: Vec<(usize, i32)> = Vec::with_capacity(temperatures.len() / 2);
        for (idx, &temperature) in temperatures.iter().enumerate() {
            while !stack.is_empty() && temperature > stack.last().unwrap().1 {
                let pop_index = stack.pop().unwrap().0;
                res[pop_index] = (idx - pop_index) as i32;
            }
            stack.push((idx, temperature));
        }
        res
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test1() {
        let temperatures = vec![73, 74, 75, 71, 69, 72, 76, 73];
        let expect = vec![1, 1, 4, 2, 1, 1, 0, 0];
        assert_eq!(Solution::daily_temperatures(temperatures), expect);
    }

    #[test]
    fn test2() {
        let temperatures = vec![30];
        let expect = vec![0];
        assert_eq!(Solution::daily_temperatures(temperatures), expect);
    }
}
