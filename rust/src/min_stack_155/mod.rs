struct MinStack {
    stack: Vec<i32>
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    fn new() -> Self {
        Self {
            stack: vec!()
        }
    }
    
    fn push(&mut self, val: i32) {
        self.stack.push(val);
    }
    
    fn pop(&mut self) {
        self.stack.pop();
    }
    
    fn top(&self) -> i32 {
        return self.stack.last().unwrap().clone();
    }
    
    fn get_min(&self) -> i32 {
        self.stack.iter().min().unwrap().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let mut min_stack = MinStack::new();
        min_stack.push(-2);
        min_stack.push(0);
        min_stack.push(-3);
        assert_eq!(-3, min_stack.get_min());
        min_stack.pop();
        assert_eq!(0, min_stack.top());
        assert_eq!(-2, min_stack.get_min());
    }
}
