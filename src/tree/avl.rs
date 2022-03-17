// TODO: define struct Balance
use std::cmp;
use std::cmp::Ordering;
use std::mem;

pub struct AVL<T: Ord> {
    root: NodeOption<T>,
}

struct NodeOption<T: Ord> {
    node_opt: Option<Node<T>>,
}

struct Node<T: Ord> {
    value: T,
    lhs: Box<NodeOption<T>>,
    rhs: Box<NodeOption<T>>,
}

impl<T: Ord> AVL<T> {
    /// Constructs a new, empty AVL<T>.
    pub fn new() -> Self {
        Self {
            root: NodeOption::new(),
        }
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
        self.root.find(value)
    }

    /// Adds a node containing a given value.
    pub fn add(&mut self, value: T) {
        self.root.add(value);
    }

    /// Removes a node containing a given value.
    ///
    /// If the value is found then the node containing it is removed and Option::Some
    /// is returned, containing the matching value. If the value is not found then
    /// Option::None is returned.
    pub fn remove(&mut self, value: &T) -> Option<T> {
        self.root.remove(value)
    }
}

impl<T: Ord> NodeOption<T> {
    fn new() -> Self {
        Self { node_opt: None }
    }

    fn from_node(node: Node<T>) -> Self {
        Self {
            node_opt: Some(node),
        }
    }

    fn find(&self, value: &T) -> Option<&T> {
        let node = self.as_ref()?;

        match value.cmp(&node.value) {
            Ordering::Equal => Some(&node.value),
            Ordering::Less => node.lhs.find(value),
            Ordering::Greater => node.rhs.find(value),
        }
    }

    fn add(&mut self, value: T) {
        match self.as_mut() {
            Some(node) => match value.cmp(&node.value) {
                Ordering::Less => {
                    node.lhs.add(value);
                    self.rebalance();
                }
                Ordering::Greater => {
                    node.rhs.add(value);
                    self.rebalance();
                }
                Ordering::Equal => (),
            },
            None => self.set(Node::new(value)),
        }
    }

    fn remove(&mut self, value: &T) -> Option<T> {
        if let Some(ref mut node) = self.as_mut() {
            let result = match value.cmp(&node.value) {
                Ordering::Less => {
                    let result = Self::remove(&mut node.lhs, value);
                    self.rebalance();

                    result
                }
                Ordering::Greater => {
                    let result = Self::remove(&mut node.rhs, value);
                    self.rebalance();

                    result
                }
                Ordering::Equal => self.remove_self(value),
            };

            return result;
        }

        None
    }

    fn rebalance(&mut self) {
        if let Some(node) = self.as_mut() {
            let balance = node.get_balance();

            match balance {
                2 => {
                    let rhs = (*node.rhs).as_ref().expect("The rhs of the node must be Option::Some because the balance of the node is positive.");
                    if rhs.get_balance() == -1 {
                        node.rhs.rotate_right();
                    }
                    self.rotate_left();
                }
                -2 => {
                    let lhs = (*node.lhs).as_ref().expect("The lhs of the node must be Option::Some because the balance of the node is negative.");
                    if lhs.get_balance() == 1 {
                        node.lhs.rotate_left();
                    }
                    self.rotate_right();
                }
                _ => (),
            }
        }
    }

    // Removes the root of tree (self).
    // The value of the node and the given value are required to be same.
    fn remove_self(&mut self, value: &T) -> Option<T> {
        let mut node = self
            .take()
            .expect("The node is required not to be Option::None.");

        match ((*node.lhs).as_mut(), (*node.rhs).as_ref()) {
            (None, None) => {
                self.set(node);
                self.take().map(|n| n.value)
            }
            (Some(_), None) => {
                if let Some(lhs) = node.lhs.take() {
                    self.set(lhs);
                }

                Some(node.value)
            }
            (None, Some(_)) => {
                if let Some(rhs) = node.rhs.take() {
                    self.set(rhs);
                }

                Some(node.value)
            }
            (Some(lhs), Some(_)) => {
                let lhs_max_node = lhs.max_mut();
                mem::swap(&mut node.value, &mut lhs_max_node.value);

                let result = Self::remove(&mut node.lhs, value);
                self.set(node);
                self.rebalance();
                result
            }
        }
    }

    // The node and the right child are required to be Option::Some.
    fn rotate_left(&mut self) {
        let mut node = self
            .take()
            .expect("The given node is required not to be None.");

        let mut rhs = node
            .rhs
            .take()
            .expect("The rhs of the given node is required not to be None.");

        if let Some(n) = rhs.lhs.take() {
            node.rhs.set(n);
        }
        rhs.lhs.set(node);

        *self = NodeOption::from_node(rhs);
    }

    // The node and the left child are required to be Option::Some.
    fn rotate_right(&mut self) {
        let mut node = self
            .take()
            .expect("The given node is required not to be None.");

        let mut lhs = node
            .lhs
            .take()
            .expect("The lhs of The given node is required not to be None.");

        if let Some(d) = lhs.rhs.take() {
            node.lhs.set(d);
        }
        lhs.rhs.set(node);

        *self = NodeOption::from_node(lhs);
    }

    // Returns the height of the given node.
    // If the node is None, then 0 is returned.
    // If the node is Some, then adding one to the bigger one of the children's heights.
    fn get_height(&self) -> i32 {
        match self.as_ref() {
            Some(node) => cmp::max(node.lhs.get_height(), node.rhs.get_height()) + 1,
            None => 0,
        }
    }

    fn as_mut(&mut self) -> Option<&mut Node<T>> {
        self.node_opt.as_mut()
    }

    fn as_ref(&self) -> Option<&Node<T>> {
        self.node_opt.as_ref()
    }

    fn take(&mut self) -> Option<Node<T>> {
        self.node_opt.take()
    }

    fn set(&mut self, node: Node<T>) {
        self.node_opt = Some(node);
    }
}

impl<T: Ord> Node<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            lhs: Box::new(NodeOption::new()),
            rhs: Box::new(NodeOption::new()),
        }
    }

    // Returns the difference of the children's heights.
    // If the right child is taller than left, then the return value is positive.
    // If the left child is taller than right, then the return value is negative.
    fn get_balance(&self) -> i32 {
        self.rhs.get_height() - self.lhs.get_height()
    }

    // Returns the mutable reference to the node containing the greatest value in the tree.
    fn max_mut(&mut self) -> &mut Self {
        match self.rhs.node_opt {
            Some(ref mut rhs) => rhs.max_mut(),
            None => self,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NodeOption;
    use super::AVL;
    use std::cmp::Ordering;

    const COMPLEX_TREE_SOURCE: [i32; 9] = [7, 5, 4, 2, 6, 11, 9, 10, 13];

    fn is_valid_balance<T: Ord>(node_opt: &NodeOption<T>) -> bool {
        if let Some(ref node) = node_opt.node_opt {
            return match node.get_balance().cmp(&1) {
                Ordering::Greater => false,
                _ => is_valid_balance(&node.lhs) && is_valid_balance(&node.rhs),
            };
        }

        true
    }

    fn is_valid_structure<T: Ord>(node_opt: &NodeOption<T>) -> bool {
        if let Some(ref node) = node_opt.node_opt {
            if let Some(ref lhs) = node.lhs.node_opt {
                if !((lhs.value < node.value) && is_valid_structure(&node.lhs)) {
                    return false;
                }
            }

            if let Some(ref rhs) = node.rhs.node_opt {
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
