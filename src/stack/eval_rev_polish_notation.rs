use crate::Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = vec![];

        tokens.iter().for_each(|s| match s.as_str() {
            "+" | "-" | "*" | "/" => {
                let val2 = stack.pop().unwrap();
                let val1 = stack.pop().unwrap();
                stack.push(Self::eval_op(s.to_string(), val1, val2).to_string())
            }
            _ => stack.push(s.to_string()),
        });
        stack.pop().unwrap().parse().unwrap()
    }

    fn eval_op(op: String, val1: String, val2: String) -> i32 {
        let val1: i32 = val1.parse().unwrap();
        let val2: i32 = val2.parse().unwrap();
        match op.as_str() {
            "+" => val1 + val2,
            "-" => val1 - val2,
            "*" => val1 * val2,
            "/" => val1 / val2,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;
    #[test]
    fn test1() {
        let tokens = ["2", "1", "+", "3", "*"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expect = 9;
        assert_eq!(Solution::eval_rpn(tokens), expect);
    }
    #[test]
    fn test2() {
        let tokens = ["4", "13", "5", "/", "+"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expect = 6;
        assert_eq!(Solution::eval_rpn(tokens), expect);
    }
    #[test]
    fn test3() {
        let tokens = [
            "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let expect = 22;
        assert_eq!(Solution::eval_rpn(tokens), expect);
    }
}
