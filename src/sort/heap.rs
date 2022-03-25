struct Heap<T: Ord> {
    data: Vec<T>, 
}
    
impl<T: Ord> Heap<T> {
    pub fn new(data: Vec<T>) -> Self {
        Self { data }
    }

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
