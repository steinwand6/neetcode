#[allow(unused)]
struct MinStack {
    stack: Vec<i32>,
    min: Option<i32>,
    size: usize,
}

impl MinStack {
    #[allow(unused)]
    fn new() -> Self {
        Self {
            stack: vec![],
            min: None,
            size: 0,
        }
    }

    #[allow(unused)]
    fn push(&mut self, val: i32) {
        self.size += 1;
        self.stack.push(val);
    }

    #[allow(unused)]
    fn pop(&mut self) {
        self.size -= 1;
        self.stack.pop();
    }

    #[allow(unused)]
    fn top(&mut self) -> i32 {
        self.stack[self.size - 1]
    }

    #[allow(unused)]
    fn get_min(&mut self) -> i32 {
        *self.stack.iter().min().unwrap()
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
