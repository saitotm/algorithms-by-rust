pub struct BinaryTree<T> {
    value: T,
}

impl<T> BinaryTree<T> {
    fn new() -> Self {
        unimplemented!("BinaryTree::new is not implemted yet!");
    }

    fn find(&self, value: &T) -> Option<&T> {
        unimplemented!("BinaryTree::find is not implemted yet!");
    }

    fn add(&mut self, value: T) {
        unimplemented!("BinaryTree::add is not implemted yet!");
    }

    fn remove(&mut self, value: &T) -> bool {
        unimplemented!("BinaryTree::add is not implemted yet!");
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
        assert_eq!(binary_tree.find(&5), Some(&3));
        assert_eq!(binary_tree.find(&7), Some(&7));
        assert_eq!(binary_tree.find(&9), None);

        binary_tree.remove(&3);

        assert_eq!(binary_tree.find(&3), None);
        assert_eq!(binary_tree.find(&5), Some(&3));
        assert_eq!(binary_tree.find(&7), Some(&7));
    }
}