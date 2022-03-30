fn binary_search<T: Ord>(array: &[T], x: &T) -> Option<usize> {
    unimplemented!(); 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_array_with_one_element() {
        let array = vec![1];

        assert_eq!(binary_search(&array, &1), Some(0));
        assert_eq!(binary_search(&array, &5), None);
    }

    #[test]
    fn search_array_with_multi_elements() {
        let array = vec![-12, -3, 1, 3, 8, 10, 21, 105];

        for (i, val) in array.iter().enumerate() {
            assert_eq!(binary_search(&array, val), Some(i));
        }

        assert_eq!(binary_search(&array, &-13), None);
        assert_eq!(binary_search(&array, &9), None);
        assert_eq!(binary_search(&array, &106), None);
    }
}
