use crate::Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for c in s.chars() {
            match c {
                '(' | '[' | '{' => stack.push(c),
                ')' | ']' | '}' => {
                    if let Some(p1) = stack.pop() {
                        if !Self::is_valid_parentheses(p1, c) {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                _ => (),
            }
        }
        stack.len() == 0
    }

    fn is_valid_parentheses(p1: char, p2: char) -> bool {
        match p1 {
            '(' => p2 == ')',
            '[' => p2 == ']',
            '{' => p2 == '}',
            _ => false,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test1() {
        let s = String::from("()[]{}");
        assert!(Solution::is_valid(s));
    }

    #[test]
    fn test2() {
        let s = String::from("(a{b [d ]}ihgah)");
        assert!(Solution::is_valid(s));
    }

    #[test]
    fn test3() {
        let s = String::from("({])");
        assert!(!Solution::is_valid(s));
    }

    #[test]
    fn test4() {
        let s = String::from("(");
        assert!(!Solution::is_valid(s));
    }
}
