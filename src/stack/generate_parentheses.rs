use crate::Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let limit = (2 as i32).pow(n as u32 * 2) as u32;
        let mut stack: Vec<String> = vec![];

        for l in 2..limit {
            let mut topbit = limit >> 1;
            let mut s = String::with_capacity(n as usize * 2);
            let mut count = 0;
            while topbit > 0 && count >= 0 && count <= n {
                if l & topbit != 0 {
                    s.push('(');
                    count += 1;
                } else {
                    s.push(')');
                    count -= 1;
                }
                topbit = topbit >> 1;
            }
            if count == 0 {
                stack.push(s);
            }
        }
        stack
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test1() {
        let n = 3;
        let mut expect = vec!["((()))", "(()())", "(())()", "()(())", "()()()"];
        let mut result = Solution::generate_parenthesis(n);
        result.sort();
        expect.sort();
        assert_eq!(result, expect);
    }

    #[test]
    fn test2() {
        let n = 1;
        let mut expect = vec!["()"];
        let mut result = Solution::generate_parenthesis(n);
        result.sort();
        expect.sort();
        assert_eq!(result, expect);
    }

    #[test]
    fn test3() {
        let n = 4;
        let mut expect = vec![
            "(((())))", "((()()))", "((())())", "((()))()", "(()(()))", "(()()())", "(()())()",
            "(())(())", "(())()()", "()((()))", "()(()())", "()(())()", "()()(())", "()()()()",
        ];
        let mut result = Solution::generate_parenthesis(n);
        result.sort();
        expect.sort();
        assert_eq!(result, expect);
    }
}
