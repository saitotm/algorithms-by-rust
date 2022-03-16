use std::cmp;
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

impl<T: Ord> AVL<T> {
    /// Constructs a new, empty AVL<T>.
    pub fn new() -> Self {
        Self { root: None }
    }

    /// Creates a AVL<T> from  slice
    pub fn from_slice(array: &[T]) -> Self
    where
        T: Copy,
    {
        array.iter().fold(Self::new(), |mut avl, v| {
            avl.add(*v);
            avl
        })
    }

    /// Finds a node for a given value.
    ///
    /// If the value is found then Option::Some is returned, containing the matching
    /// value. If the value is not found then Option::None is returned.
    pub fn find(&self, value: &T) -> Option<&T> {
        self.root.as_ref()?.find(value)
    }

    /// Adds a node containing a given value.
    pub fn add(&mut self, value: T) {
        Node::add(&mut self.root, value);
    }

    /// Removes a node containing a given value.
    ///
    /// If the value is found then the node containing it is removed and Option::Some
    /// is returned, containing the matching value. If the value is not found then
    /// Option::None is returned.
    pub fn remove(&mut self, value: &T) -> Option<T> {
        Node::remove(&mut self.root, value)
    }
}

impl<T: Ord> Node<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            lhs: None,
            rhs: None,
        }
    }

    fn get_balance(&self) -> i32 {
        Self::get_height(&self.rhs) - Self::get_height(&self.lhs)
    }

    fn get_height(node_opt: &NodeOpt<T>) -> i32 {
        match node_opt {
            Some(node) => cmp::max(Self::get_height(&node.lhs), Self::get_height(&node.rhs)) + 1,
            None => 0,
        }
    }

    fn find(&self, value: &T) -> Option<&T> {
        match value.cmp(&self.value) {
            Ordering::Equal => Some(&self.value),
            Ordering::Less => self.lhs.as_ref()?.find(value),
            Ordering::Greater => self.rhs.as_ref()?.find(value),
        }
    }

    fn rotate_left(node_opt: &mut NodeOpt<T>) {
        let mut node = node_opt.take().unwrap();
        let mut rhs = node.rhs.take().unwrap();

        node.rhs = rhs.lhs.take();
        rhs.lhs = Some(node);

        *node_opt = Some(rhs);
    }

    fn rotate_right(node_opt: &mut NodeOpt<T>) {
        let mut node = node_opt.take().unwrap();
        let mut lhs = node.lhs.take().unwrap();

        node.lhs = lhs.rhs.take();
        lhs.rhs = Some(node);

        *node_opt = Some(lhs);
    }

    fn rebalance(node_opt: &mut NodeOpt<T>) {
        if let Some(ref mut node) = node_opt {
            let balance = node.get_balance();

            match balance {
                2 => {
                    let rhs = node.rhs.as_ref().unwrap();
                    if rhs.get_balance() == -1 {
                        Self::rotate_right(&mut node.rhs);
                    }
                    Self::rotate_left(node_opt);
                }
                -2 => {
                    let lhs = node.lhs.as_ref().unwrap();
                    if lhs.get_balance() == 1 {
                        Self::rotate_left(&mut node.lhs);
                    }
                    Self::rotate_right(node_opt);
                }
                _ => (),
            }
        }
    }

    fn add(node_opt: &mut NodeOpt<T>, value: T) {
        match node_opt {
            Some(ref mut node) => match value.cmp(&node.value) {
                Ordering::Less => {
                    Node::add(&mut node.lhs, value);
                    Self::rebalance(node_opt);
                }
                Ordering::Greater => {
                    Node::add(&mut node.rhs, value);
                    Self::rebalance(node_opt);
                }
                Ordering::Equal => (),
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
            }
            (Some(_), None) => {
                let lhs = node.lhs.take();
                *node_opt = lhs;

                Some(node.value)
            }
            (None, Some(_)) => {
                let rhs = node.rhs.take();
                *node_opt = rhs;

                Some(node.value)
            }
            (Some(lhs), Some(_)) => {
                let lhs_max_node = lhs.max();
                mem::swap(&mut node.value, &mut lhs_max_node.value);

                let result = Self::remove(&mut node.lhs, value);
                *node_opt = Some(node);
                Self::rebalance(node_opt);
                result
            }
        }
    }

    fn remove(node_opt: &mut NodeOpt<T>, value: &T) -> Option<T> {
        if let Some(ref mut node) = node_opt {
            let result = match value.cmp(&node.value) {
                Ordering::Less => {
                    let result = Self::remove(&mut node.lhs, value);
                    Self::rebalance(node_opt);

                    result
                }
                Ordering::Greater => {
                    let result = Self::remove(&mut node.rhs, value);
                    Self::rebalance(node_opt);

                    result
                }
                Ordering::Equal => Self::remove_self(node_opt, value),
            };

            return result;
        }

        None
    fn max(&mut self) -> &mut Self {
        match self.rhs {
            Some(ref mut rhs) => rhs.max(),
            None => self,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NodeOpt;
    use super::AVL;
    use std::cmp::Ordering;

    const COMPLEX_TREE_SOURCE: [i32; 9] = [7, 5, 4, 2, 6, 11, 9, 10, 13];

    fn is_valid_balance<T: Ord>(node_opt: &NodeOpt<T>) -> bool {
        if let Some(node) = node_opt {
            return match node.get_balance().cmp(&1) {
                Ordering::Greater => false,
                _ => is_valid_balance(&node.lhs) && is_valid_balance(&node.rhs),
            };
        }

        true
    }

    fn is_valid_structure<T: Ord>(node_opt: &NodeOpt<T>) -> bool {
        if let Some(node) = node_opt {
            if let Some(ref lhs) = node.lhs {
                if !((lhs.value < node.value) && is_valid_structure(&node.lhs)) {
                    return false;
                }
            }

            if let Some(ref rhs) = node.rhs {
                if !((rhs.value > node.value) && is_valid_structure(&node.rhs)) {
                    return false;
                }
            }
        }

        true
    }

    fn test_find(nums: &[i32]) {
        let binary_tree = AVL::from_slice(nums);

        assert!(is_valid_balance(&binary_tree.root));
        assert!(is_valid_structure(&binary_tree.root));
        assert_eq!(binary_tree.find(&3), None);

        nums.iter().for_each(|n| {
            assert_eq!(binary_tree.find(n), Some(n));
        });
    }

    fn test_remove(nums: &[i32]) {
        for removed_num in nums {
            let mut binary_tree = AVL::from_slice(nums);

            assert!(is_valid_balance(&binary_tree.root));
            assert!(is_valid_structure(&binary_tree.root));
            nums.iter()
                .for_each(|n| assert_eq!(binary_tree.find(n), Some(n)));

            assert_eq!(binary_tree.remove(removed_num), Some(*removed_num));

            assert!(is_valid_balance(&binary_tree.root));
            assert!(is_valid_structure(&binary_tree.root));

            nums.iter().for_each(|n| match n.cmp(removed_num) {
                Ordering::Equal => assert_eq!(binary_tree.find(removed_num), None),
                _ => assert_eq!(binary_tree.find(n), Some(n)),
            });
        }
    }

    #[test]
    fn find_empty_tree() {
        let binary_tree = AVL::new();
        assert_eq!(binary_tree.find(&3), None);
    }

    #[test]
    fn find_root_only_tree() {
        let nums = [7];
        test_find(&nums[..]);
    }

    #[test]
    fn find_root_and_lhs_only_tree() {
        let nums = [7, 2];
        test_find(&nums[..]);
    }

    #[test]
    fn find_root_and_rhs_only_tree() {
        let nums = [7, 11];
        test_find(&nums[..]);
    }

    #[test]
    fn find_complex_tree() {
        test_find(&COMPLEX_TREE_SOURCE[..]);
    }

    #[test]
    fn remove_empty_tree() {
        let mut binary_tree = AVL::new();
        assert_eq!(binary_tree.remove(&7), None);
        assert_eq!(binary_tree.find(&7), None);
    }

    #[test]
    fn remove_root_only_tree() {
        let nums = [7];
        test_remove(&nums[..]);
    }

    #[test]
    fn remove_root_and_lhs_only_tree() {
        let nums = [7, 2];
        test_remove(&nums[..]);
    }

    #[test]
    fn remove_root_and_rhs_only_tree() {
        let nums = [7, 11];
        test_remove(&nums[..]);
    }

    #[test]
    fn remove_complex_tree() {
        let nums = COMPLEX_TREE_SOURCE;
        test_remove(&nums[..]);
    }
}
