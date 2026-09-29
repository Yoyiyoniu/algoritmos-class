use std::fmt::Debug;

pub struct Stack<T> {
    elements: Vec<T>,
}

impl<T: Debug> Stack<T> {
    pub fn new() -> Self {
        Stack {
            elements: Vec::new(),
        }
    }

    pub fn push(&mut self, item: T) {
        self.elements.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.elements.last()
    }

    // pub fn is_empty(&self) -> bool {
    //     self.elements.is_empty()
    // }

    // pub fn remove(&mut self, removed: &T) -> bool
    // where
    //     T: PartialEq,
    // {
    //     match self.elements.iter().rposition(|e| e == removed) {
    //         Some(pos) => {
    //             self.elements.remove(pos);
    //             true
    //         }
    //         None => false,
    //     }
    // }

    pub fn print_all(&self) {
        self.elements.iter().for_each(|i| {
            println!("{:?}", i);
        });
    }

    // pub fn len(&self) -> usize {
    //     self.elements.len()
    // }

    pub fn reverce(&mut self) {
        self.elements.reverse();
    }
}
