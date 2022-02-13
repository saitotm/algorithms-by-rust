struct AVL<T: Ord> {
    root: NodeOpt<T>,
}

struct Node<T: Ord> {
    value: T,
    lhs: NodeOpt<T>,
    rhs: NodeOpt<T>,
}

type NodeOpt<T> = Option<Box<Node<T>>>;

impl<T: Ord> AVL<T> {
    fn new() -> Self {
        unimplemented!("AVL::new is not implemented!");
    }

    fn make_tree(array: &[T]) -> Self where
        T: Copy {
        unimplemented!("AVL::make_tree is not implemented!");
    }

    fn find(&self, value: &T) -> Option<&T> {
        unimplemented!("AVL::find is not implemented!");
    }

    fn add(&mut self, value: T) {
        unimplemented!("AVL::add is not implemented!");
    }

    fn remove(&mut self, value: &T) -> Option<T> {
        unimplemented!("AVL::remove is not implemented!");
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