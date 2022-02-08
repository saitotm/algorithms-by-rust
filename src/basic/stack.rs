pub struct Stack<T> {
    cur: Option<Box<Node<T>>>,
} 

struct Node<T> {
    value: T,
    prev: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T, prev: Option<Box<Node<T>>>) -> Self {
        Self { value, prev }
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
        self.cur = Some(Box::new(Node::new(value, self.cur.take())));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.cur.take()
        .map(|cur| {
            self.cur = cur.prev;
            cur.value
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