use std::cmp::Ordering;

pub struct BinaryTree<T: Ord> {
    root: Option<Box<Node<T>>>,
}

struct Node<T: Ord> {
    value: T,
    lhs: Option<Box<Self>>,
    rhs: Option<Box<Self>>,
}


impl<T: Ord> Node<T> {
    fn new(value: T) -> Self {
        Self { value: value, lhs: None, rhs: None }
    }

    fn find(&self, value: &T) -> Option<&Self> {
        match value.cmp(&self.value) {
            Ordering::Equal => Some(self),
            Ordering::Less => self.lhs.as_ref()?.find(value),
            Ordering::Greater => self.rhs.as_ref()?.find(value),
        }
    }

    fn add(&mut self, value: T) {
        match value.cmp(&self.value) {
            Ordering::Less => { 
                match self.lhs.as_mut() {
                    Some(lhs) => lhs.add(value),
                    None => { self.lhs = Some(Box::new(Node::new(value))) },
                }
            },
            Ordering::Greater => {
                match self.rhs.as_mut() {
                    Some(rhs) => rhs.add(value),
                    None => { self.rhs = Some(Box::new(Node::new(value))) },
                }
            },
            Ordering::Equal => return,
        };
    }

    fn take_child_min(&mut self) -> Option<Box<Node<T>>> {
        match self.lhs.as_mut() {
            Some(lhs) => {
                match lhs.lhs {
                    Some(_) => lhs.take_child_min(),
                    None => self.lhs.take(),
                }
            },
            None => self.rhs.take(),
        }
    }

    fn remove_child(&mut self, value: &T) {
        match value.cmp(&self.value) {
            Ordering::Less => {
                if let Some(mut lhs) = self.lhs.take() {
                    match value.cmp(&lhs.value) {
                        Ordering::Equal => {
                            self.lhs = lhs.take_child_min()
                            .map(|mut new_lhs| {
                                new_lhs.lhs = lhs.lhs;
                                new_lhs.rhs = lhs.rhs;
                                new_lhs
                            });
                        },
                        _ => {
                            lhs.remove_child(value);
                            self.lhs = Some(lhs);
                        }
                    }
                }
            },
            Ordering::Greater => {
                if let Some(mut rhs) = self.rhs.take() {
                    match value.cmp(&rhs.value) {
                        Ordering::Equal => {
                            self.rhs = rhs.take_child_min()
                            .map(|mut new_rhs| {
                                new_rhs.lhs = rhs.lhs;
                                new_rhs.rhs = rhs.rhs;
                                new_rhs
                            });
                        },
                        _ => {
                            rhs.remove_child(value);
                            self.rhs = Some(rhs);
                        }
                    }
                }
            },
            _ => { /* do not remove self */ },
        }
    }
}

impl<T: Ord> BinaryTree<T> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn find(&self, value: &T) -> Option<&T> {
        self.root.as_ref()?.find(value).map(|n| &n.value)
    }

    pub fn add(&mut self, value: T) {
        match self.root.as_mut() {
            Some(root) => { root.add(value) },
            None => { self.root = Some(Box::new(Node::new(value))) },
        }
    }

    pub fn remove(&mut self, value: &T) -> bool {
        match self.root.as_mut() {
            Some(root) => match value.cmp(&root.value) {
                Ordering::Equal => self.root = root.take_child_min(),
                _ => root.remove_child(value),
            },
            None => (),
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::BinaryTree;

    #[test]
    fn test_binary_tree() {
        let mut binary_tree = BinaryTree::new();
        assert_eq!(binary_tree.find(&3), None);

        binary_tree.add(7);
        binary_tree.add(5);
        binary_tree.add(3);

        assert_eq!(binary_tree.find(&3), Some(&3));
        assert_eq!(binary_tree.find(&5), Some(&5));
        assert_eq!(binary_tree.find(&7), Some(&7));
        assert_eq!(binary_tree.find(&9), None);

        binary_tree.remove(&3);

        assert_eq!(binary_tree.find(&7), Some(&7));
        assert_eq!(binary_tree.find(&5), Some(&5));
        assert_eq!(binary_tree.find(&3), None);
    }
}