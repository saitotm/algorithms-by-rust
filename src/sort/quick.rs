use std::{fmt::Debug, cmp::Ordering};


// If any element in array is equal to any other elment, then return None. 
// Otherwise, return the index refering to the element that is bigger than the an element in the array.
fn select_pivot<T: Ord + Debug>(array: &[T]) -> Option<usize> {
    if array.is_empty() {
        return None;
    }

    let top = &array[0];

    for (i, value) in array.iter().enumerate().skip(1) {
        match top.cmp(value) {
            Ordering::Less => return Some(i),
            Ordering::Greater => return Some(0),
            Ordering::Equal => (),
        }
    }

    None
}


// Partition the array by the given pivot.
// Return (Vec<T>, Vec<T>). The elements in the left vector are small than the element which is referred to by pivot_idx.
// The elements in the right vector are equal to or more than the element which is referred by
// pivot_idx.
//
// The element referred by pivot_idx must be greater than an element in the given array.
fn partition<T: Ord + Debug> (mut array: Vec<T>, mut pivot_idx: usize) -> (Vec<T>, Vec<T>) {
    let mut left = 0;
    let mut right = array.len() - 1;

    loop {
        while array[left] < array[pivot_idx] {
            left += 1;
        }

        while array[right] >= array[pivot_idx] {
            if right == 0 {
                panic!("pivot_idx");
            } 

            right -= 1;
        }

        if left < right {
            array.swap(left, right);

            if pivot_idx == left {
                pivot_idx = right;
            } else if pivot_idx == right {
                pivot_idx = left;
            }

        } else {
            let right_vec = array.split_off(left);
            return (array, right_vec);
        }
    }
}

fn sort<T: Ord + Debug>(array: Vec<T>) -> Vec<T> {
    match select_pivot(&array) {
        None => array,
        Some(pivot) => {
            let (mut left, mut right) = partition(array, pivot);

            if left.is_empty() {
                return right;
            }

            left = sort(left);
            right = sort(right);

            left.append(&mut right);

            left
        },
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn partition_numbers() {
        let data = vec![2, 9, 4, 10, 3];

        assert_eq!(partition(data, 1), (vec![2, 3, 4], vec![10, 9]));
    }


    #[test]
    fn partition_continuaous_numbers() {
        let data = vec![5, 9, 7, 2, 3, 4, 1, 10, 6, 8];

        assert_eq!(partition(data, 1), (vec![5, 8, 7, 2, 3, 4, 1, 6], vec![10, 9]));
    }

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
