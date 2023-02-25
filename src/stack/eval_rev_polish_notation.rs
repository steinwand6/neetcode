use crate::Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = vec![];
        for token in tokens.iter() {
            match token.as_str() {
                "+" | "-" | "*" | "/" => {
                    Self::eval_op_and_push(&token, &mut stack);
                }
                _ => stack.push(token.parse().unwrap()),
            }
        }
        stack.pop().unwrap()
    }

    #[inline]
    fn eval_op_and_push(op: &str, stack: &mut Vec<i32>) {
        let val2 = stack.pop().unwrap();
        let val1 = stack.pop().unwrap();

        stack.push(match op {
            "+" => val1 + val2,
            "-" => val1 - val2,
            "*" => val1 * val2,
            "/" => val1 / val2,
            _ => unreachable!(),
        });
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
