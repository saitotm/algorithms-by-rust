use std::{fmt::Debug, cmp::Ordering};

fn is_sorted<T: Ord>(array: &[T]) -> bool {
    if array.is_empty() {
        return true;
    }

    for w in array.windows(2) {
        if w[0] > w[1] {
            return false;
        }
    }

    true
}

fn split<T: Ord + Debug>(array: &mut Vec<T>) -> Vec<T> {
    debug_assert!(array.len() >= 2);

    let at = array.len() / 2;
    let output = array.split_off(at);


    debug_assert!(!array.is_empty());
    debug_assert!(!output.is_empty());
    output
}

fn merge<T: Ord + Debug>(mut array_x: Vec<T>, mut array_y: Vec<T>) -> Vec<T> {
    debug_assert!(!array_x.is_empty());
    debug_assert!(!array_y.is_empty());
    debug_assert!(is_sorted(&array_x));
    debug_assert!(is_sorted(&array_y));

    let length_x = array_x.len();
    let length_y = array_y.len();
    
    array_x.reverse();
    array_y.reverse();

    let mut output = Vec::new();

    while !array_x.is_empty() || !array_y.is_empty() {
        match (array_x.last(), array_y.last()) {
            (Some(x), Some(y)) => {
                match x.cmp(y) {
                    Ordering::Less => { output.push(array_x.pop().expect("must be Some")) },
                    _ => { output.push(array_y.pop().expect("must be Some")) },
                }
            },
            (Some(_), None) => { output.push(array_x.pop().expect("must be Some")) },
            (None, Some(_)) => { output.push(array_y.pop().expect("must be Some")) },
            (None, None,) => (),
        }

        debug_assert!(is_sorted(&output));
    }

    debug_assert!(is_sorted(&output));
    debug_assert_eq!(output.len(), length_x + length_y);
    output
}

fn sort<T: Ord + Debug>(array: Vec<T>) -> Vec<T> {
    let length = array.len();
    if length < 2 {
        return array;
    }

    let mut array_x = array;
    let array_y = split(&mut array_x);

    let array_x = sort(array_x);
    let array_y = sort(array_y);

    let output = merge(array_x, array_y);

    debug_assert!(is_sorted(&output));
    debug_assert_eq!(output.len(), length);
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_one_number() {
        let data = vec![3];

        assert_eq!(sort(data), vec![3]);
    }

    #[test]
    fn sort_two_numbers() {
        let data = vec![3, 2];

        assert_eq!(sort(data), vec![2, 3]);
    }

    #[test]
    fn sort_numbers() {
        let data = vec![2, 9, 4, 10, 3];

        assert_eq!(sort(data), vec![2, 3, 4, 9, 10]);
    }

    #[test]
    fn sort_continuous_numbers() {
        let data = vec![1, 4, 9, 7, 2, 3, 5, 10, 6, 8];

        assert_eq!(sort(data), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn sort_same_numbers() {
        let data = vec![1, 3, 2, 4, 1, 3];

        assert_eq!(sort(data), vec![1, 1, 2, 3, 3, 4]);
    }

    #[test]
    fn sort_same_numbers2() {
        let data = vec![1, 3, 1, 4, 4, 1, 3, 2, 4, 1, 3, 1, 2, 3, 1, 3];

        assert_eq!(sort(data), vec![1, 1, 1, 1, 1, 1, 2, 2, 3, 3, 3, 3, 3, 4, 4, 4]);
    }
}
