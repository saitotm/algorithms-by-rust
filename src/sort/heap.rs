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
    fn sort1() {
        unimplemented!() 
    }
}
