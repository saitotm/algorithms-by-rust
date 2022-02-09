use std::rc::Rc;
use std::cell::RefCell;

pub struct Queue<T: Default> {
    head: Rc<RefCell<Node<T>>>,
}

struct Node<T> {
    value: Option<Rc<T>>,
    next: Option<Rc<RefCell<Self>>>,
}

impl<T: Default> Node<T> {
    fn new(value: Option<T>) -> Self {
        Self { value: value.map(|v| Rc::new(v)), next: None }
    }
}

impl<T: Default> Queue<T> {
    pub fn new() -> Self {
        Self { head: Rc::new(RefCell::new(Node::new(None))) }
        //Self { head: Box::new(Node::default()) }
    }

    pub fn peek(&self) -> Option<Rc<T>> {
        let cur = &self.head.borrow().next;

        match cur {
            Some(c) => {
                let c = c.borrow();
                let x = Some(c.value.as_ref().map(|x| x.clone()).unwrap());

                x
            }
            None => None,
        }
        //self.head.borrow().next.map(|cur| cur.borrow().value)
    }

    pub fn push(&mut self, value: T) {
        let mut node = self.head.clone();

        loop {
            match &node.clone().borrow().next {
                Some(n) => { node = n.clone() },
                None => {   
                    break;
                },
            }
        }

        let new_node = Some(Rc::new(RefCell::new(Node::new(Some(value)))));
        node.borrow_mut().next = new_node;
    }

    pub fn pop(&mut self) -> Option<Rc<T>> {
        let mut head = self.head.borrow_mut();
        head.next.take()
        .map(|ret| {
            let ret = ret.borrow();
            head.next = ret.next.as_ref().map(|r| r.clone());

            ret.value.as_ref().map(|r| r.clone()).unwrap()
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

        assert_eq!(que.peek(), Some(Rc::new(5)));

        assert_eq!(que.pop(), Some(Rc::new(5)));
        assert_eq!(que.pop(), Some(Rc::new(2)));
        assert_eq!(que.pop(), Some(Rc::new(9)));
        assert_eq!(que.pop(), None);
        assert_eq!(que.pop(), None);
    }
}