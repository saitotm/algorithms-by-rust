use std::rc::Rc;
use std::cell::RefCell;

pub struct Queue<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
}

struct Node<T> {
    value: Rc<T>,
    next: Option<Rc<RefCell<Self>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Self { value: Rc::new(value), next: None }
    }
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn peek(&self) -> Option<Rc<T>> {
        self.head.as_ref().map(|h| h.borrow().value.clone())
    }

    pub fn push(&mut self, value: T) {
        let new_node = Some(Rc::new(RefCell::new(Node::new(value))));

        match self.head {
            None => self.head = new_node,
            Some(ref h) => {
                let mut node = h.clone();

                loop {
                    match &node.clone().borrow().next {
                        Some(n) => { node = n.clone() },
                        None => {   
                            break;
                        },
                    }
                }

                node.borrow_mut().next = new_node;
            },
        }
    }

    pub fn pop(&mut self) -> Option<Rc<T>> {
        self.head.take()
        .map(|ret| {
            let ret = ret.borrow();
            self.head = ret.next.clone();

            ret.value.clone()
        })
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use super::Queue;

    #[test] 
    fn test_queue() {
        let mut que = Queue::new();

        que.push(5);
        que.push(2);
        que.push(9);

        let que_peek_5 = que.peek();
        assert_eq!(que_peek_5, Some(Rc::new(5)));

        assert_eq!(que.pop(), Some(Rc::new(5)));
        assert_eq!(que.pop(), Some(Rc::new(2)));
        assert_eq!(que.pop(), Some(Rc::new(9)));
        assert_eq!(que.pop(), None);
        assert_eq!(que.pop(), None);

        assert_eq!(que_peek_5, Some(Rc::new(5)));
    }
}