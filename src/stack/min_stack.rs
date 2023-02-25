#[allow(unused)]
struct MinStack {
    stack: Vec<i32>,
    mins: Vec<i32>,
}

impl MinStack {
    #[allow(unused)]
    fn new() -> Self {
        Self {
            stack: vec![],
            mins: vec![],
        }
    }

    #[allow(unused)]
    fn push(&mut self, val: i32) {
        if self.mins.is_empty() || *self.mins.last().unwrap() >= val {
            self.mins.push(val);
        }
        self.stack.push(val);
    }

    #[allow(unused)]
    fn pop(&mut self) {
        let val = self.stack.pop();
        if *self.mins.last().unwrap() == val.unwrap() {
            self.mins.pop();
        }
    }

    #[allow(unused)]
    fn top(&mut self) -> i32 {
        *self.stack.last().unwrap()
    }

    #[allow(unused)]
    fn get_min(&mut self) -> i32 {
        *self.mins.last().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::MinStack;

    #[test]
    fn test() {
        let mut minstack = MinStack::new();
        minstack.push(-2);
        minstack.push(0);
        minstack.push(-3);
        assert_eq!(minstack.get_min(), -3);
        minstack.pop();
        assert_eq!(minstack.top(), 0);
        assert_eq!(minstack.get_min(), -2);
    }
}
