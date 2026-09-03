pub struct Stack<T, const N: usize> {
    pub stack: [Option<T>; N],
    pub count: usize,
}

impl<T, const N: usize> Stack<T, N> {
    pub fn new() -> Self {
        Stack {
            stack: std::array::from_fn(|_| None),
            count: 0,
        }
    }

    pub fn push(&mut self, item: T) -> Result<(), &str> {
        if self.count >= N {
            return Err("Stack overflow");
        }
        self.stack[self.count] = Some(item);
        self.count += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.count == 0 {
            return None;
        }
        self.count -= 1;
        self.stack[self.count].take()
    }

    pub fn peek(&self) -> Option<&T> {
        if self.count == 0 {
            return None;
        }

        self.stack[self.count - 1].as_ref()
    }
}
