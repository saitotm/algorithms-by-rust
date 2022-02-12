use std::mem;
use std::cmp::Ordering;

pub struct BinaryTree<T: Ord> {
    root: Option<Box<Node<T>>>,
}

#[derive(Debug)]
struct Node<T: Ord> {
    value: T,
    lhs: Option<Box<Self>>,
    rhs: Option<Box<Self>>,
}


impl<T: Ord + std::fmt::Debug> Node<T> {
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

    fn remove(node_opt: &mut Option<Box<Self>>, value: &T) -> Option<Box<Self>> {
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

impl<T: Ord + std::fmt::Debug> BinaryTree<T> where{
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

    // Todo: return true when the node which contains the value is removed.
    pub fn remove(&mut self, value: &T) -> bool {
        match Node::remove(&mut self.root, value) {
            Some(_) => true,
            None => false,
        }
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

    //        7
    //      /    \
    //     5      11
    //    / \    /  \
    //   4   6  9    13
    //  /        \   
    // 2          10
    const COMPLEX_TREE_SOURCE: [i32; 9] = [7, 5, 4, 2, 6, 11, 9, 10, 13];

    #[test]
    fn test_find_empty_tree() {
        let binary_tree = BinaryTree::new();
        assert_eq!(binary_tree.find(&3), None);
    }

    #[test]
    fn test_find_root_only_tree() {
        let mut binary_tree = BinaryTree::new();
        binary_tree.add(7);

        assert_eq!(binary_tree.find(&3), None);
        assert_eq!(binary_tree.find(&7), Some(&7));
    }

    #[test]
    fn test_find_root_and_lhs_only_tree() {
        let mut binary_tree = BinaryTree::new();
        binary_tree.add(7);
        binary_tree.add(2);

        assert_eq!(binary_tree.find(&3), None);
        assert_eq!(binary_tree.find(&7), Some(&7));
        assert_eq!(binary_tree.find(&2), Some(&2));
    }

    #[test]
    fn test_find_root_and_rhs_only_tree() {
        let mut binary_tree = BinaryTree::new();
        binary_tree.add(7);
        binary_tree.add(11);

        assert_eq!(binary_tree.find(&3), None);
        assert_eq!(binary_tree.find(&7), Some(&7));
        assert_eq!(binary_tree.find(&11), Some(&11));
    }

    #[test]
    fn test_find_complex_tree() {
        let binary_tree = BinaryTree::make_tree(&COMPLEX_TREE_SOURCE);

        assert_eq!(binary_tree.find(&3), None);

        COMPLEX_TREE_SOURCE
        .into_iter()
        .for_each(|n| {
            assert_eq!(binary_tree.find(&n), Some(&n));
        });
    }

    #[test]
    fn test_remove_empty_tree() {
        let mut binary_tree = BinaryTree::new();
        assert!(!binary_tree.remove(&7));
        assert_eq!(binary_tree.find(&7), None);
    }

    #[test]
    fn test_remove_root_only_tree() {
        let mut binary_tree = BinaryTree::new();
        binary_tree.add(7);
        assert_eq!(binary_tree.find(&7), Some(&7));

        assert!(binary_tree.remove(&7));
        assert_eq!(binary_tree.find(&7), None);
    }

    #[test]
    fn test_remove_root_and_lhs_only_tree() {
        let nums = [7, 2];
        for removed_num in nums {
            let mut binary_tree = BinaryTree::make_tree(&nums);

            nums
            .into_iter()
            .for_each(|n|
                assert_eq!(binary_tree.find(&n), Some(&n))
            );

            assert!(binary_tree.remove(&removed_num));
            
            nums
            .into_iter()
            .for_each(|n| {
                match n.cmp(&removed_num) {
                    Ordering::Equal => assert_eq!(binary_tree.find(&removed_num), None),
                    _ => assert_eq!(binary_tree.find(&n), Some(&n)),
                }
            });
        }
    }
     
    #[test]
    fn test_remove_complex_tree() {
        let nums = COMPLEX_TREE_SOURCE;
        for removed_num in nums {
            dbg!(&removed_num);
            let mut binary_tree = BinaryTree::make_tree(&nums);

            nums
            .into_iter()
            .for_each(|n|
                assert_eq!(binary_tree.find(&n), Some(&n))
            );

            assert!(binary_tree.remove(&removed_num));
            
            nums
            .into_iter()
            .for_each(|n| {
                match n.cmp(&removed_num) {
                    Ordering::Equal => assert_eq!(binary_tree.find(&removed_num), None),
                    _ => { 
                        assert_eq!(binary_tree.find(&n), Some(&n));
                    },
                }
            });
        }
    }

    #[test]
    fn test_remove_root_and_rhs_only_tree() {
        let nums = [7, 11];
        for removed_num in nums {
            let mut binary_tree = BinaryTree::make_tree(&nums);

            nums
            .into_iter()
            .for_each(|n| 
                assert_eq!(binary_tree.find(&n), Some(&n))
            );

            assert!(binary_tree.remove(&removed_num));

            nums
            .into_iter()
            .for_each(|n| {
                match n.cmp(&removed_num) {
                    Ordering::Equal => assert_eq!(binary_tree.find(&removed_num), None),
                    _ => assert_eq!(binary_tree.find(&n), Some(&n)),
                }
            });
        }
    }


    // Todo: Add more test
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