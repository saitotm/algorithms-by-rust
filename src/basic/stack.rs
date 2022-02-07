use std::rc::Rc;

pub struct Stack<T> {
    cur: Option<Rc<Node<T>>>,
} 

struct Node<T> {
    value: T,
    prev: Option<Rc<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T, prev: Option<Rc<Node<T>>>) -> Self {
        Self { value, prev }
    }
}

impl<T: Clone> Clone for Node<T> {
    fn clone(&self) -> Self {
        Self { value: self.value.clone(), prev: self.prev.clone() }
    }
}

impl<T> Clone for Stack<T> {
    fn clone(&self) -> Self {
        Self{ cur: self.cur.clone() }
    }
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self{cur: None}
    }
    
    pub fn peek(&self) -> Option<&T> {
        self.cur.as_ref().map(|rc| &rc.value)
    }

    pub fn push(&mut self, value: T) {
        self.cur = Some(Rc::new(Node::new(value, self.cur.take())));
    }
}

impl<T: Clone> Stack<T> {
    pub fn pop(&mut self) -> Option<T> {
        self.cur.take()
        .map(|rc| {
            let top = Rc::try_unwrap(rc).unwrap_or_else(|rc| (*rc).clone());
            self.cur = top.prev;
            top.value
        })
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