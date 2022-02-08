pub struct Queue<T> {
    value: T
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        unimplemented!("Queue::new is not implemented yet!");
    }

    pub fn peek(&self) -> Option<&T> {
        unimplemented!("Queue::peek is not implemented yet!");
    }

    pub fn push(&mut self, value: T) {
        unimplemented!("Queue::push is not implemented yet!");
    }

    pub fn pop(&mut self) -> Option<T> {
        unimplemented!("Queue::pop is not implemented yet!");
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

        assert_eq!(que.peek(), Some(&5));

        assert_eq!(que.pop(), Some(5));
        assert_eq!(que.pop(), Some(2));
        assert_eq!(que.pop(), Some(9));
        assert_eq!(que.pop(), None);
        assert_eq!(que.pop(), None);
    }
}