/// Looking for the maximum element.
///
pub fn max<T>(list: &[T]) -> Option<usize>
where
    T: PartialOrd,
{
    if list.is_empty() {
        return None;
    }
    let mut max_idx = 0;
    let mut max_val = &list[max_idx];
    for idx in 0..list.len() {
        if list[idx] > *max_val {
            max_idx = idx;
            max_val = &list[idx];
        }
    }
    Some(max_idx)
}

/// Looking for the minimum element.
///
pub fn min<T>(list: &[T]) -> Option<usize>
where
    T: PartialOrd,
{
    if list.is_empty() {
        return None;
    }
    let mut min_idx = 0;
    let mut min_val = &list[min_idx];
    for idx in 0..list.len() {
        if list[idx] < *min_val {
            min_idx = idx;
            min_val = &list[idx];
        }
    }
    Some(min_idx)
}

/// Binary search in slice.
///
pub fn binary<T>(list: &[T], item: T) -> Option<usize>
where
    T: PartialOrd,
{
    if list.is_empty() {
        return None;
    }

    let mut lhs = 0;
    let mut rhs = list.len() - 1;

    while lhs <= rhs {
        let mid = (lhs + rhs) / 2;
        if list[mid] > item {
            rhs = mid - 1;
        } else if list[mid] < item {
            lhs = mid + 1;
        } else {
            return Some(mid);
        }
    }
    None
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn max_ok() {
        let list = [10, 12, 45, 23, 29, 100, 1000, 0, 12];
        assert_eq!(max(&list).unwrap(), 6);

        let list: &[i32] = &[];
        assert_eq!(max(list), None);
    }

    #[test]
    fn min_ok() {
        let list = [10, 12, 45, 23, 29, 100, 1000, 0, 12];
        assert_eq!(min(&list).unwrap(), 7);

        let list: &[i32] = &[];
        assert_eq!(min(list), None);
    }

    mod binary {

        #[test]
        fn binary_integer_ok() {
            let list = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
            assert_eq!(super::binary(&list, 7).unwrap(), 6);

            let list = [5];
            assert_eq!(super::binary(&list, 5).unwrap(), 0);

            let list = [5, 5, 5];
            assert_eq!(super::binary(&list, 5).unwrap(), 1);

            let list = [];
            assert_eq!(super::binary(&list, 0), None);
        }

        #[test]
        fn binary_float_ok() {
            let list = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
            assert_eq!(super::binary(&list, 7.0).unwrap(), 6);

            let list = [1.000, 1.0005, 1.001, 1.0015, 1.0016, 2.0];
            assert_eq!(super::binary(&list, 1.0015).unwrap(), 3);
        }

        #[test]
        fn binary_str_ok() {
            let list = ["a", "a", "b", "e", "f"];
            assert_eq!(super::binary(&list, "e").unwrap(), 3);

            let list = ["aaa", "aab", "abb", "abc", "bcd"];
            assert_eq!(super::binary(&list, "aab").unwrap(), 1);
        }

        #[test]
        fn binary_string_ok() {
            let list = [
                "a".to_string(),
                "a".to_string(),
                "b".to_string(),
                "e".to_string(),
                "f".to_string(),
            ];
            assert_eq!(super::binary(&list, "e".to_string()).unwrap(), 3);

            let list = [
                "aaa".to_string(),
                "aab".to_string(),
                "abb".to_string(),
                "abc".to_string(),
                "bcd".to_string(),
            ];
            assert_eq!(super::binary(&list, "aab".to_string()).unwrap(), 1);
        }
    }
}
