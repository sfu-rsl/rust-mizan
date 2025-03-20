use super::search;

/// Selection sort.
///
pub fn selection<T>(list: &mut [T], ascending: bool)
where
    T: PartialOrd,
{
    if list.len() < 2 {
        return;
    }

    let mut current = 0;
    let upper = list.len() - 1;

    while current < upper {
        let found = if ascending {
            search::min(&list[current..])
        } else {
            search::max(&list[current..])
        }
        .unwrap()
            + current;

        if compare(&list[found], &list[current], ascending) {
            list.swap(current, found);
        }

        current += 1;
    }
}

/// Quick sort.
///
pub fn quick<T>(list: &mut [T], ascending: bool)
where
    T: PartialOrd,
{
    if list.len() < 2 {
        return;
    }

    recursion(list, 0, list.len() - 1, ascending);

    fn recursion<T: PartialOrd>(list: &mut [T], lhs: usize, rhs: usize, ascending: bool) {
        let pivot = (lhs + rhs) / 2;
        let mut i = lhs;
        let mut j = rhs;

        while i <= j {
            while compare(&list[i], &list[pivot], ascending) {
                i += 1
            }
            while compare(&list[j], &list[pivot], !ascending) {
                j -= 1
            }

            if i <= j {
                list.swap(i, j);
                if i < rhs {
                    i += 1;
                }
                if j > lhs {
                    j -= 1
                };
            }
        }

        if j > lhs {
            recursion(list, lhs, j, ascending)
        }
        if i < rhs {
            recursion(list, i, rhs, ascending)
        }
    }
}

fn compare<T: PartialOrd>(a: &T, b: &T, ascending: bool) -> bool {
    if ascending {
        a < b
    } else {
        a > b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selection_ok() {
        let mut list = [1, 2, 12, 5, 43, 21, 0, 2, 34, 100, 3];

        selection(&mut list, true);
        assert_ascending(&list);

        selection(&mut list, false);
        assert_descending(&list);
    }

    #[test]
    fn quick_ok() {
        let mut list = [1, 2, 12, 5, 43, 21, 0, 2, 34, 100, 3];
        quick(&mut list, true);
        assert_ascending(&list);

        let mut list = [1, 2, 12, 5, 43, 21, 0, 2, 34, 100, 3];
        quick(&mut list, false);
        assert_descending(&list);

        let mut list = [1, 2, 3, 4, 5, 6, 7];
        quick(&mut list, true);
        assert_ascending(&list);

        let mut list = [7, 6, 5, 4, 3, 2, 1];
        quick(&mut list, true);
        assert_ascending(&list);

        let mut list = [1, 1, 1, 4, 2, 2, 2];
        quick(&mut list, true);
        assert_ascending(&list);

        let mut list = [5, 5, 5, 1, 5, 5, 5];
        quick(&mut list, true);
        assert_ascending(&list);

        let mut list = [7];
        quick(&mut list, true);
        assert_ascending(&list);

        let mut list: [i32; 0] = [];
        quick(&mut list, true);
    }

    fn assert_ascending<T: PartialOrd>(list: &[T]) {
        for i in 0..list.len() - 1 {
            assert!(list[i] <= list[i + 1]);
        }
    }

    fn assert_descending<T: PartialOrd>(list: &[T]) {
        for i in 0..list.len() - 1 {
            assert!(list[i] >= list[i + 1]);
        }
    }
}
