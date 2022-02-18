use std::mem;
use std::cmp::Ordering;
use std::fmt::Debug;

pub struct BinaryTree<T: Ord> {
    root: NodeOpt<T>,
}

#[derive(Debug)]
struct Node<T: Ord> {
    value: T,
    lhs: NodeOpt<T>,
    rhs: NodeOpt<T>,
}

type NodeOpt<T> = Option<Box<Node<T>>>;


impl<T: Ord + Debug> Node<T> {
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

    fn min_mut(&mut self) -> &mut Self {
        match self.lhs {
            Some(ref mut lhs) => lhs.min_mut(),
            None => self,
        }
    }

    fn max_mut(&mut self) -> &mut Self {
        match self.rhs {
            Some(ref mut rhs) => rhs.max_mut(),
            None => self,
        }
    }

    fn remove(node_opt: &mut NodeOpt<T>, value: &T) -> NodeOpt<T> {
        let mut node = node_opt.take()?;

        match value.cmp(&node.value) {
            Ordering::Equal => {
                match (&mut node.lhs, &node.rhs) {
                    (None, None) => {
                        Some(node)
                    },
                    (Some(lhs), Some(_)) => {
                        let max_node = lhs.max_mut();
                        mem::swap(&mut node.value, &mut max_node.value);

                        let removed_node_opt = Self::remove(&mut node.lhs, value);
                        *node_opt = Some(node);
                        removed_node_opt
                    },
                    (None, Some(_)) => {
                        let mut removed_node = node;
                        *node_opt = removed_node.rhs.take();
                        Some(removed_node)
                    },
                    (Some(_), None) => {
                        let mut removed_node = node;
                        *node_opt = removed_node.lhs.take();
                        Some(removed_node)
                    },
                }
            },
            Ordering::Less => { 
                let removed_node = Self::remove(&mut node.lhs, value);
                *node_opt = Some(node);
                removed_node
            },
            Ordering::Greater => {
                let removed_node = Self::remove(&mut node.rhs, value);
                *node_opt = Some(node);
                removed_node
            },
        }
    }
}

impl<T: Ord + Debug> BinaryTree<T> where{
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn make_tree(array: &[T]) -> Self where
        T: Copy {

        array
        .into_iter()
        .fold(Self::new(), |mut tree, v| {
                tree.add(*v);
                tree
            }
        )
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

    pub fn remove(&mut self, value: &T) -> Option<T> {
        Node::remove(&mut self.root, value).map(|node| node.value)
    }
}

macro_rules! binary_tree {
    ( $($x : expr),* ) => {
        {
            let mut temp_bt = BinaryTree::new();
            $(
                temp_bt.add($x);
            )*
            temp_bt
        }
    };
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use super::BinaryTree;
    use super::NodeOpt;

    //        7
    //      /    \
    //     5      11
    //    / \    /  \
    //   4   6  9    13
    //  /        \   
    // 2          10
    const COMPLEX_TREE_SOURCE: [i32; 9] = [7, 5, 4, 2, 6, 11, 9, 10, 13];

    fn is_valid_structure<T: Ord>(node_opt: &NodeOpt<T>) -> bool {
        if let Some(node) = node_opt {
            if let Some(ref lhs) = node.lhs {
                if !( (lhs.value < node.value) && is_valid_structure(&node.lhs) ) {
                    return false;
                }
            }

            if let Some(ref rhs) = node.rhs {
                if !( (rhs.value > node.value) && is_valid_structure(&node.rhs) ) {
                    return false;
                }
            }
        }

        true
    }

    fn test_find(nums: &[i32]) {
        let binary_tree = BinaryTree::make_tree(nums);

        assert!(is_valid_structure(&binary_tree.root));
        assert_eq!(binary_tree.find(&3), None);

        for n in nums {
            assert_eq!(binary_tree.find(n), Some(n));
        }
    }

    fn test_remove(nums: &[i32]) {
        for removed_num in nums {
            let mut binary_tree = BinaryTree::make_tree(& nums);

            assert!(is_valid_structure(&binary_tree.root));

            for n in nums {
                assert_eq!(binary_tree.find(n), Some(n));
            }

            assert_eq!(binary_tree.remove(&removed_num), Some(*removed_num));

            assert!(is_valid_structure(&binary_tree.root));
            
            for n in nums {
                match n.cmp(&removed_num) {
                    Ordering::Equal => assert_eq!(binary_tree.find(&removed_num), None),
                    _ => assert_eq!(binary_tree.find(n), Some(n)),
                }
            }
        }
    }

    #[test]
    fn test_find_empty_tree() {
        let binary_tree = BinaryTree::new();
        assert_eq!(binary_tree.find(&3), None);
    }

    #[test]
    fn test_find_root_only_tree() {
        let nums = [7];
        test_find(&nums[..]);
    }

    #[test]
    fn test_find_root_and_lhs_only_tree() {
        let nums = [7, 2];
        test_find(&nums[..]);
    }

    #[test]
    fn test_find_root_and_rhs_only_tree() {
        let nums = [7, 11];
        test_find(&nums[..]);
    }

    #[test]
    fn test_find_complex_tree() {
        test_find(&COMPLEX_TREE_SOURCE[..]);
    }

    #[test]
    fn test_remove_empty_tree() {
        let mut binary_tree = BinaryTree::new();
        assert_eq!(binary_tree.remove(&7), None);
        assert_eq!(binary_tree.find(&7), None);
    }

    #[test]
    fn test_remove_root_only_tree() {
        let nums = [7];
        test_remove(&nums[..]);
    }

    #[test]
    fn test_remove_root_and_lhs_only_tree() {
        let nums = [7, 2];
        test_remove(&nums[..]);
    }

    #[test]
    fn test_remove_root_and_rhs_only_tree() {
        let nums = [7, 11];
        test_remove(&nums[..]);
    }

    #[test]
    fn test_remove_complex_tree() {
        let nums = COMPLEX_TREE_SOURCE;
        test_remove(&nums[..]);
    }
}