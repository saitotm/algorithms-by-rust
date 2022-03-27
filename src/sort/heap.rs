struct Node<'a, T: Ord> {
    heap: &'a mut Heap<T>,
    index: usize,
}

fn lhs_index(index: usize) -> usize {
    2 * index + 1
}

fn rhs_index(index: usize) -> usize {
    2 * index + 2
}

impl<'a, T: Ord> Node<'a, T> {
    fn new(heap: &'a mut Heap<T>, index: usize) -> Self {
        Self { heap, index }
    }

    fn get(&self) -> &T {
        self.heap.get(self.index).expect("must be Some")
    }

    fn get_lhs_value(&self) -> Option<&T> {
        let index = lhs_index(self.index);
        self.heap.get(index)
    }

    fn get_rhs_value(&self) -> Option<&T> {
        let index = rhs_index(self.index);
        self.heap.get(index)
    }

    fn get_lhs_mut(&mut self) -> Option<Node<'_, T>> {
        let index = lhs_index(self.index);

        if self.heap.is_valid_index(index) {
            return Some(Node::new(self.heap, index));
        }

        None
    }

    fn get_rhs_mut(&mut self) -> Option<Node<'_, T>> {
        let index = rhs_index(self.index);

        if self.heap.is_valid_index(index) {
            return Some(Node::new(self.heap, index));
        }

        None
    }

    fn swap_with_lhs(&mut self) {
        let lhs = lhs_index(self.index);
        self.heap.data.swap(self.index, lhs);
    }

    fn swap_with_rhs(&mut self) {
        let rhs = rhs_index(self.index);
        self.heap.data.swap(self.index, rhs);
    }

    fn fix_root(&'a mut self) {
        let root_value = self.get();

        match (self.get_lhs_value(), self.get_rhs_value()) {
            (Some(lvalue), Some(rvalue)) if root_value < lvalue && rvalue < lvalue => {
                self.swap_with_lhs();

                if let Some(mut lhs) = self.get_lhs_mut() {
                    lhs.fix_root();
                }
            }

            (Some(_), Some(rvalue)) if root_value < rvalue => {
                self.swap_with_rhs();

                if let Some(mut rhs) = self.get_rhs_mut() {
                    rhs.fix_root();
                }
           }

            (Some(lvalue), None) if root_value < lvalue => {
                self.swap_with_lhs();

                if let Some(mut lhs) = self.get_lhs_mut() {
                    lhs.fix_root();
                }
            }

            (None, Some(rvalue)) if root_value < rvalue => {
                self.swap_with_rhs();

                if let Some(mut rhs) = self.get_rhs_mut() {
                    rhs.fix_root();
                }
            }

            _ => (),
        }
    }
}

struct Heap<T: Ord> {
    data: Vec<T>,
    tree_size: usize,
}

impl<T: Ord> Heap<T> {
    pub fn new(data: Vec<T>) -> Self {
        let tree_size = data.len();
        let mut heap = Self { data, tree_size };

        heap.reshape();
        heap
    }

    pub fn sort(mut self) -> Vec<T> {
        for i in (1..self.tree_size).rev() {
            self.swap_root_and_tail();
            self.get_node(0).fix_root();
        }

        self.data
    }

    fn swap_root_and_tail(&mut self) {
        let tail_index = self.get_tail_index();
        self.data.swap(0, tail_index);

        self.tree_size -= 1;
    }

    fn get_tail_index(&self) -> usize {
        self.tree_size - 1
    }

    fn get_node(&mut self, index: usize) -> Node<'_, T> {
        if !self.is_valid_index(index) {
            panic!("out of range");
        }

        Node::new(self, index)
    }

    fn reshape(&mut self) {
        for i in (0..self.tree_size).rev() {
            self.get_node(i).fix_root()
        }
    }

    fn get(&self, index: usize) -> Option<&T> {
        if self.is_valid_index(index) {
            return self.data.get(index)
        }

        None
    }

    fn is_valid_index(&self, index: usize) -> bool {
        index < self.tree_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_valid_heap<T: Ord>(node: &mut Node<'_, T>) -> bool {
        if let Some(lhs) = node.get_lhs_value() {
            if node.get() < lhs {
                return false;
            }
        }

        if let Some(mut lhs) = node.get_lhs_mut() {
            if !is_valid_heap(&mut lhs) {
                return false;
            }
        }

        if let Some(rhs) = node.get_rhs_value() {
            if node.get() < rhs {
                return false;
            }
        }

        if let Some(mut rhs) = node.get_rhs_mut() {
            if !is_valid_heap(&mut rhs) {
                return false;
            }
        }

        true
    }

    #[test]
    fn make_valid_heap() {
        let data = vec![1, 3, 2, 4, 1, 3];
        let mut heap = Heap::new(data);

        assert!(is_valid_heap(&mut heap.get_node(0)));
    }

    #[test]
    fn swap_test() {
        let data = vec![1, 3, 2, 4, 1, 3];
        let mut heap = Heap::new(data);
        dbg!(heap.get_tail_index());
        dbg!(&heap.data);


        heap.swap_root_and_tail();
        dbg!(heap.get_tail_index());
        heap.get_node(0).fix_root();

        dbg!(&heap.data);
        assert!(is_valid_heap(&mut heap.get_node(0)));
    }

    #[test]
    fn sort_one_number() {
        let data = vec![3];
        let heap = Heap::new(data);

        assert_eq!(heap.sort(), vec![3]);
    }

    #[test]
    fn sort_two_numbers() {
        let data = vec![3, 2];
        let heap = Heap::new(data);

        assert_eq!(heap.sort(), vec![2, 3]);
    }

    #[test]
    fn sort_numbers() {
        let data = vec![2, 9, 4, 10, 3];
        let heap = Heap::new(data);

        assert_eq!(heap.sort(), vec![2, 3, 4, 9, 10]);
    }

    #[test]
    fn sort_continuous_numbers() {
        let data = vec![1, 4, 9, 7, 2, 3, 5, 10, 6, 8];
        let heap = Heap::new(data);

        assert_eq!(heap.sort(), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn sort_same_numbers() {
        let data = vec![1, 3, 2, 4, 1, 3];
        let heap = Heap::new(data);

        assert_eq!(heap.sort(), vec![1, 1, 2, 3, 3, 4]);
    }
}
