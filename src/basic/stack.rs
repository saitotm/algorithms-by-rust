use std::rc::Rc;

pub struct Stack<T>(Option<Rc<(T, Stack<T>)>>);

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
            Some(&rc.0)
        } else {
            None
        }
    }

    pub fn push(self, value: T) -> Self {
        Self(Some(Rc::new((value, self))))
    }
}

impl<T: Clone> Stack<T> {
    pub fn pop(self) -> (Self, Option<T>) {
        if let Some(rc) = self.0 {
            let (head, tail) = Rc::try_unwrap(rc).unwrap_or_else(|rc| (*rc).clone());
            (tail, Some(head))
        } else {
            (Self(None), None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Stack;

    #[test]
    fn test_stack() {
        let stack = Stack::new();
        let stack = stack.push(5);
        let stack = stack.push(2);
        let mut stack = stack.push(9);

        let (stack, head) = stack.pop();
        assert_eq!(head, Some(9));

        let (stack, head) = stack.pop();
        assert_eq!(head, Some(2));

        let (stack, head) = stack.pop();
        assert_eq!(head, Some(5));

        let (stack, head) = stack.pop();
        assert_eq!(head, None);

        let (stack, head) = stack.pop();
        assert_eq!(head, None);
    }
}