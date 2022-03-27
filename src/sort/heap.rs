struct Node<'a, T: Ord> {
    heap: &'a mut Heap<T>,
    index: usize,
}

fn lhs_index(index: usize) -> usize {
    2 * index
}

fn rhs_index(index: usize) -> usize {
    2 * index + 1
}

impl<'a, T: Ord> Node<'a, T> {
    fn new(heap: &'a mut Heap<T>, index: usize) -> Self {
        Self { heap, index }
    }

    fn get(&self) -> &T {
        self.heap.get(self.index).expect("must be Some")
    }

    fn get_rhs_value(&self) -> Option<&T> {
        let index = rhs_index(self.index);
        self.heap.get(index)
    }

    fn get_lhs_value(&self) -> Option<&T> {
        let index = lhs_index(self.index);
        self.heap.get(index)
    }

    fn get_lhs_mut(self) -> Option<Self> {
        let index = lhs_index(self.index);

        if self.heap.is_valid_index(index) {
            return Some(Self::new(self.heap, index));
        }

        None
    }

    fn get_rhs_mut(self) -> Option<Self> {
        let index = rhs_index(self.index);

        if self.heap.is_valid_index(index) {
            return Some(Self::new(self.heap, index));
        }

        None
    }
    fn swap_with_lhs(&mut self) {
        let left = lhs_index(self.index);
        self.heap.data.swap(self.index, left);
    }

    fn swap_with_rhs(&mut self) {
        let left = rhs_index(self.index);
        self.heap.data.swap(self.index, left);
    }

    fn fix_root(self) {
        let root_value = self.get();

        match (self.get_lhs_value(), self.get_rhs_value()) {
            (Some(lvalue), Some(rvalue)) if root_value < lvalue && rvalue < lvalue => {
                self.swap_with_lhs();

                if let Some(lhs) = self.get_lhs_mut() {
                    lhs.fix_root();
                }
            }

            (Some(lvalue), Some(rvalue)) if root_value < rvalue => {
                self.swap_with_rhs();

                if let Some(lhs) = self.get_rhs_mut() {
                    lhs.fix_root();
                }
            }

            (Some(lvalue), None) if root_value < lvalue => {
                self.swap_with_lhs();

                if let Some(lhs) = self.get_lhs_mut() {
                    lhs.fix_root();
                }
            }

            (None, Some(rvalue)) if root_value < rvalue => {
                self.swap_with_rhs();

                if let Some(lhs) = self.get_rhs_mut() {
                    lhs.fix_root();
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
        let mut heap = Self { data };
        heap.reshape();

        heap
    }

    fn get_node<'a>(&'a mut self, index: usize) -> Node<'a, T> {
        Node::new(&mut self, index)
    }

    fn reshape(&mut self) {
        for i in (0..self.len()).rev() {
            self.get_node(i).fix_root()
        }
    }

    fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    fn get_left(&self, index: usize) -> Option<&T> {
        self.data.get(Self::left_index(index))
    }

    fn get_right(&self, index: usize) -> Option<&T> {
        self.data.get(Self::right_index(index))
    }

    fn left_index(parent: usize) -> usize {
        2 * parent
    }

    fn right_index(parent: usize) -> usize {
        2 * parent + 1
    }

    fn is_valid_index(&self, index: usize) -> bool {
        index < self.tree_size
    }

    fn make_heap(&mut self) {}

    pub fn sort(&mut self) {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_one_number() {
        let data = vec![3];
        let mut heap = Heap::new(data);

        heap.sort();
        assert_eq!(heap.data, vec![3]);
    }

    #[test]
    fn sort_two_numbers() {
        let data = vec![3, 2];
        let mut heap = Heap::new(data);

        heap.sort();
        assert_eq!(heap.data, vec![2, 3]);
    }

    #[test]
    fn sort_numbers() {
        let data = vec![2, 9, 4, 10, 3];
        let mut heap = Heap::new(data);

        heap.sort();
        assert_eq!(heap.data, vec![2, 3, 4, 9, 10]);
    }

    #[test]
    fn sort_continuous_numbers() {
        let data = vec![1, 4, 9, 7, 2, 3, 5, 10, 6, 8];
        let mut heap = Heap::new(data);

        heap.sort();
        assert_eq!(heap.data, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn sort_same_numbers() {
        let data = vec![1, 3, 2, 4, 1, 3];
        let mut heap = Heap::new(data);

        heap.sort();
        assert_eq!(heap.data, vec![1, 1, 2, 3, 3, 4]);
    }
}
