pub struct Queue<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    value: T,
    next: Option<Box<Self>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Self { value: value, next: None }
    }

    fn push(&mut self, value: T) {
        match self.next {
            Some(ref mut next) => { next.push(value) },
            None => { self.next = Some(Box::new(Node::new(value))) },
        }
    }
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|h| &h.value)
    }

    pub fn push(&mut self, value: T) {
        match self.head {
            None => self.head = Some(Box::new(Node::new(value))),
            Some(ref mut h) => h.push(value),
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take()
        .map(|ret| {
            self.head = ret.next;

            ret.value
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Queue;

    #[test] 
    fn test_queue() {
        let mut que = Queue::new();

        que.push(5);
        que.push(2);
        que.push(9);

        let que_peek_5 = que.peek();
        assert_eq!(que_peek_5, Some(&5));

        assert_eq!(que.pop(), Some(5));
        assert_eq!(que.pop(), Some(2));
        assert_eq!(que.pop(), Some(9));
        assert_eq!(que.pop(), None);
        assert_eq!(que.pop(), None);
    }
}
