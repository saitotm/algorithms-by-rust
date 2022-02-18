use std::cmp::Ordering;
use std::mem;

struct AVL<T: Ord> {
    root: NodeOpt<T>,
}

struct Node<T: Ord> {
    value: T,
    lhs: NodeOpt<T>,
    rhs: NodeOpt<T>,
}

type NodeOpt<T> = Option<Box<Node<T>>>;

impl<T: Ord> Node<T> {
    fn new(value: T) -> Self {
        Self { value: value, lhs: None, rhs: None }
    }

    fn find(&self, value: &T) -> Option<&T> {
        match value.cmp(&self.value) {
            Ordering::Equal => Some(&self.value),
            Ordering::Less => self.lhs.as_ref()?.find(value), 
            Ordering::Greater => self.rhs.as_ref()?.find(value), 
        }
    }

    fn add(node_opt: &mut NodeOpt<T>, value: T) {
        match node_opt {
            Some(ref mut node) => {
                match value.cmp(&node.value) {
                    Ordering::Less => Node::add(&mut node.lhs, value),
                    Ordering::Greater => Node::add(&mut node.rhs, value),
                    Ordering::Equal => (),
                }
            },
            None => *node_opt = Some(Box::new(Node::new(value))),
        }
    }

    fn min(&mut self, value: &T) -> &mut Self {
        match self.lhs {
            Some(ref mut lhs) => lhs.min(value),
            None => self,
        }
    }

    fn max(&mut self, value: &T) -> &mut Self {
        match self.rhs {
            Some(ref mut rhs) => rhs.max(value),
            None => self,
        }
    }

    fn remove_self(node_opt: &mut NodeOpt<T>, value: &T) -> Option<T> {
        let mut node = node_opt.take()?;

        match (&mut node.lhs, &node.rhs) {
            (None, None) => {
                *node_opt = Some(node);
                node_opt.take().map(|n| n.value)
            },
            (Some(_), None) => {
                let lhs = node.lhs.take();
                *node_opt = lhs;

                Some(node.value)
            },
            (None, Some(_)) => {
                let rhs = node.rhs.take();
                *node_opt = rhs;

                Some(node.value)
            },
            (Some(lhs), Some(_)) => {
                let min_node = lhs.max(value);
                mem::swap(&mut node.value, &mut min_node.value);

                let result = Self::remove(&mut node.lhs, value);
                *node_opt = Some(node);
                result
            }
        }
    }

    fn remove(node_opt: &mut NodeOpt<T>, value: &T) -> Option<T> {
        if let Some(ref mut node) = node_opt {
            return match value.cmp(&node.value) {
                Ordering::Less => Self::remove(&mut node.lhs, value),
                Ordering::Greater => Self::remove(&mut node.rhs, value),
                Ordering::Equal => Self::remove_self(node_opt, value),
            }
        }

        None
    }
}

impl<T: Ord> AVL<T> {
    fn new() -> Self {
        Self { root: None }
    }

    fn make_tree(array: &[T]) -> Self where
        T: Copy {
        array.iter().fold(Self::new(), |mut avl, v| { avl.add(*v); avl})
    }

    fn find(&self, value: &T) -> Option<&T> {
        self.root.as_ref()?.find(&value)
    }

    fn add(&mut self, value: T) {
        Node::add(&mut self.root, value);
    }

    fn remove(&mut self, value: &T) -> Option<T> {
        Node::remove(&mut self.root, value)
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use super::AVL;
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
        let binary_tree = AVL::make_tree(nums);

        assert!(is_valid_structure(&binary_tree.root));
        assert_eq!(binary_tree.find(&3), None);

        nums
        .into_iter()
        .for_each(|n| {
            assert_eq!(binary_tree.find(n), Some(n));
        });
    }

    fn test_remove(nums: &[i32]) {
        for removed_num in nums {
            let mut binary_tree = AVL::make_tree(& nums);

            assert!(is_valid_structure(&binary_tree.root));
            nums
            .into_iter()
            .for_each(|n|
                assert_eq!(binary_tree.find(n), Some(n))
            );

            assert_eq!(binary_tree.remove(&removed_num), Some(*removed_num));

            assert!(is_valid_structure(&binary_tree.root));
            
            nums
            .into_iter()
            .for_each(|n| {
                match n.cmp(&removed_num) {
                    Ordering::Equal => assert_eq!(binary_tree.find(&removed_num), None),
                    _ => assert_eq!(binary_tree.find(n), Some(n)),
                }
            });
        }
    }

    #[test]
    fn test_find_empty_tree() {
        let binary_tree = AVL::new();
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
        let mut binary_tree = AVL::new();
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