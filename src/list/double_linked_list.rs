struct DoubleLinkedList<T> {
    value: T,
}

impl<T> DoubleLinkedList<T> {
    fn new() -> Self {
        unimplemented!("DoubleLinkedList::new is not implemented");
    }

    fn get(&self, index: usize) -> Option<&T> {
        unimplemented!("DoubleLinkedList::get is not implemented");
    }

    fn set(&mut self, index: usize, value: T) -> bool {
        unimplemented!("DoubleLinkedList::set is not implemented");
    }

    fn add(&mut self, value: T) {
        unimplemented!("DoubleLinkedList::add is not implemented");
    }

    fn remove(&mut self) {
        unimplemented!("DoubleLinkedList::remove is not implemented");
    }
}

#[cfg(test)]
mod tests {
    use super::DoubleLinkedList;

    #[test]
    fn test_double_linked_list() {
        let mut dl_list = DoubleLinkedList::new();

        dl_list.add(8);
    }
}