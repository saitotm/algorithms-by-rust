struct Heap<T: Ord> {
    data: Vec<T>, 
}

impl<T: Ord> Heap<T> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
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
