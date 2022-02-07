use std::rc::Rc;

pub struct Stack<T> (Option<Rc<Item<T>>>);

struct Item<T> {
    value: T,
    prev: Stack<T>,
}

impl<T> Item<T> {
    fn new(value: T, prev: Stack<T>) -> Self {
        Self { value, prev }
    }
}

impl<T: Clone> Clone for Item<T> {
    fn clone(&self) -> Self {
        Self { value: self.value.clone(), prev: self.prev.clone() }
    }
}

impl<T> Clone for Stack<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self(None)
    }
    
    pub fn peek(&self) -> Option<&T> {
        if let Some(rc) = &self.0 {
            Some(&rc.value)
        } else {
            None
        }
    }

    pub fn push(&mut self, value: T) {
        let this = Self(self.0.take());
        self.0 = Some(Rc::new(Item::new(value, this)));
    }
}

impl<T: Clone> Stack<T> {
    pub fn pop(&mut self) -> Option<T> {
        let this = Self(self.0.take());
        if let Some(rc) = this.0 {
            let item = Rc::try_unwrap(rc).unwrap_or_else(|rc| (*rc).clone());
            *self = item.prev;
            Some(item.value)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Stack;

    #[test]
    fn test_stack() {
        let mut stack = Stack::new();
        stack.push(5);
        stack.push(2);
        stack.push(9);

        assert_eq!(stack.peek(), Some(&9));

        assert_eq!(stack.pop(), Some(9));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(5));
        assert_eq!(stack.pop(), None);
        assert_eq!(stack.pop(), None);
    }
}