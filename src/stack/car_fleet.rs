use crate::Solution;

impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut stack = Vec::with_capacity(position.len());
        //let mut sorted: Vec<(i32, i32)> = std::iter::zip(position, speed).collect();
        let mut sorted: Vec<(i32, i32)> = position.into_iter().zip(speed.into_iter()).collect();
        //sorted.sort_by(|v1, v2| (v2.0).cmp(&v1.0));
        sorted.sort_unstable_by_key(|f| -f.0);
        stack.push(sorted[0]);
        for (pos, spd) in sorted.into_iter().skip(1) {
            let (next_pos, next_spd) = stack.last().unwrap();
            let next_car_arrive_time = (target - next_pos) as f32 / (*next_spd as f32);
            let car_arrive_time = (target - pos) as f32 / spd as f32;
            if car_arrive_time > next_car_arrive_time {
                stack.push((pos, spd));
            }
        }
        stack.len() as i32
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test1() {
        let target = 12;
        let position = vec![10, 8, 0, 5, 3];
        let speed = vec![2, 4, 1, 1, 3];
        let expect = 3;
        assert_eq!(Solution::car_fleet(target, position, speed), expect);
    }
    #[test]
    fn test2() {
        let target = 10;
        let position = vec![3];
        let speed = vec![3];
        let expect = 1;
        assert_eq!(Solution::car_fleet(target, position, speed), expect);
    }
    #[test]
    fn test3() {
        let target = 100;
        let position = vec![99, 80, 20, 1];
        let speed = vec![1, 10, 30, 40];
        let expect = 3;
        assert_eq!(Solution::car_fleet(target, position, speed), expect);
    }
}
