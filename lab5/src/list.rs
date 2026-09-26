struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None, len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push_back(&mut self, value: T) {
        let node = Box::new(Node { value, next: None });
        let mut current = &mut self.head;
        while let Some(next_node) = current {
            current = &mut next_node.next;
        }
        *current = Some(node);
        self.len += 1;
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        let mut current = self.head.as_deref();
        let mut i = 0;
        while let Some(node) = current {
            if i == index {
                return Some(&node.value);
            }
            current = node.next.as_deref();
            i += 1;
        }
        None
    }

    pub fn update_at(&mut self, index: usize, update: impl FnOnce(&mut T)) -> bool {
        let mut current = &mut self.head;
        let mut i = 0;
        while let Some(node) = current {
            if i == index {
                update(&mut node.value);
                return true;
            }
            current = &mut node.next;
            i += 1;
        }
        false
    }

    pub fn remove_at(&mut self, index: usize) -> Option<T> {
        if index >= self.len {
            return None;
        }
        let mut current = &mut self.head;
        for _ in 0..index {
            match current {
                Some(node) => current = &mut node.next,
                None => return None,
            }
        }
        let node = current.take()?;
        *current = node.next;
        self.len -= 1;
        Some(node.value)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut current = self.head.take();
        while let Some(mut node) = current {
            current = node.next.take();
        }
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.value
        })
    }
}
