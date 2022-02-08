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
        let mut btree = BinaryTree::new();
        assert_eq!(btree.find(&3), None);

        btree.add(7);
        btree.add(5);
        btree.add(3);

        assert_eq!(btree.find(&3), Some(&3));
        assert_eq!(btree.find(&5), Some(&3));
        assert_eq!(btree.find(&7), Some(&7));
        assert_eq!(btree.find(&9), None);

        btree.remove(&3);

        assert_eq!(btree.find(&3), None);
        assert_eq!(btree.find(&5), Some(&3));
        assert_eq!(btree.find(&7), Some(&7));
    }
}