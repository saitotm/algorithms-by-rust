use std::rc::Rc;

pub struct Stack<T> {
    node: Option<Rc<Node<T>>>,
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
        Self{ node: self.node.clone() }
    }
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self{node: None}
    }
    
    pub fn peek(&self) -> Option<&T> {
        self.node.as_ref().map(|rc| &rc.value)
    }

    pub fn push(&mut self, value: T) {
        self.node = Some(Rc::new(Node::new(value, self.node.take())));
    }
}

impl<T: Clone> Stack<T> {
    pub fn pop(&mut self) -> Option<T> {
        self.node.take()
        .map(|rc| {
            let item = Rc::try_unwrap(rc).unwrap_or_else(|rc| (*rc).clone());
            self.node = item.prev;
            item.value
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