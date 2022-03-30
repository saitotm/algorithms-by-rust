use std::{cmp::Ordering, fmt::Debug};

fn is_sorted<T: Ord>(array: &[T]) -> bool {
    for w in array.windows(2) {
        if w[0] > w[1] {
            return false;
        }
    }

    true
}

fn binary_search<T: Ord + Debug>(array: &[T], x: &T) -> Option<usize> {
    assert!(is_sorted(array));

    if array.is_empty() {
        return None;
    }

    if array.len() == 1 {
        return match x.cmp(&array[0]) {
            Ordering::Equal => Some(0),
            _ => None,
        };
    }

    let mid = array.len() / 2;
    debug_assert!(!&array[..mid].is_empty());
    debug_assert!(!&array[mid..].is_empty());

    let index = match x.cmp(&array[mid]) {
        Ordering::Less => binary_search(&array[..mid], x),
        Ordering::Greater => binary_search(&array[mid + 1..], x).map(|idx| idx + mid + 1),
        Ordering::Equal => Some(mid),
    };

    debug_assert!(match index {
        Some(idx) => &array[idx] == x,
        _ => true,
    });
    index
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

    #[test]
    fn search_array_with_duplicate_elements() {
        let array = vec![-12, -3, -3, 1, 3, 3, 3, 8, 8, 10, 21, 21, 105];

        assert_eq!(binary_search(&array, &-12), Some(0));
        assert_eq!(binary_search(&array, &1), Some(3));
        assert_eq!(binary_search(&array, &10), Some(9));
        assert_eq!(binary_search(&array, &105), Some(12));

        assert!(matches!(binary_search(&array, &-3), Some(1..=2)));
        assert!(matches!(binary_search(&array, &3), Some(4..=6)));
        assert!(matches!(binary_search(&array, &8), Some(7..=8)));
        assert!(matches!(binary_search(&array, &21), Some(10..=11)));

        assert_eq!(binary_search(&array, &-13), None);
        assert_eq!(binary_search(&array, &9), None);
        assert_eq!(binary_search(&array, &106), None);
    }
}
