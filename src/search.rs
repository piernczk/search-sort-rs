use std::cmp::Ordering;

/// An implementation of linear search.
///
/// Looks for the value in the slice by iterating over it. Returns the position
/// of its first equal element, or [`None`] if not found.
///
/// `slice.iter().find(|&x| x == &value)` does almost the same thing.
///
/// # Examples
///
/// ```
/// use search_sort::search;
///
/// let slice = [1, 85, 23, -4, 8];
/// assert_eq!(search::linear(&slice, &23), Some(2));
/// assert_eq!(search::linear(&slice, &-77), None);
/// ```
pub fn linear<T: PartialEq>(slice: &[T], value: &T) -> Option<usize> {
    for (i, v) in slice.iter().enumerate() {
        if value == v {
            return Some(i);
        }
    }

    None
}

/// An implementation of binary search.
///
/// Recursively searches for the value in a sorted slice. It does the following:
/// * computes the center of the slice (size / 2),
/// * compares it with the value,
/// * if it's smaller, invokes itself with the first part of the slice,
/// * if they are equal, returns the center,
/// * if it's greater, invokes itself with the second part of the slice and
///     adds the current center and 1.
/// * if didn't find the value (center == 0 || center >= size - 1), returns
///     [`None`].
///
/// **Note**: the returned value is the position of the first found element,
/// that may not be the position of the first element in the whole slice. Use
/// [`binary_first`] instead.
///
/// # Examples
///
/// ```
/// use search_sort::search;
///
/// let slice = [1, 2, 4, 8, 16, 32];
/// assert_eq!(search::binary(&slice, &8), Some(3));
/// assert_eq!(search::binary(&slice, &3), None);
/// ```
pub fn binary<T: Ord>(slice: &[T], value: &T) -> Option<usize> {
    let mid = slice.len() / 2;
    match value.cmp(&slice[mid]) {
        Ordering::Less if mid > 0 => binary(&slice[0..mid], value),
        Ordering::Equal => Some(mid),
        Ordering::Greater if mid < slice.len() - 1 => {
            match binary(&slice[(mid + 1)..slice.len()], value) {
                Some(x) => Some(x + mid + 1),
                None => None,
            }
        }
        _ => None,
    }
}

/// An implementation of binary search that finds the very first position of
/// the element.
///
/// Invokes [`binary`] search and iterates over the elements backward in
/// the slice before the found element. Returns the position of the last equal
/// element.
///
/// # Examples
///
/// ```
/// use search_sort::search;
///
/// let fib = [1, 1, 2, 3];
/// // the first found element is on position 1, since it doesn't check the
/// // elements before
/// assert_eq!(search::binary(&fib, &1), Some(1));
/// assert_eq!(search::binary_first(&fib, &1), Some(0));
/// ```
pub fn binary_first<T: Ord>(slice: &[T], value: &T) -> Option<usize> {
    let pos = binary(slice, value);
    match pos {
        Some(pos) => {
            for (i, v) in slice[0..pos].iter().enumerate().rev() {
                if v < value {
                    return Some(i + 1);
                }
            }
            Some(0)
        }
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::binary;
    use super::binary_first;
    use super::linear;

    #[test]
    fn linear_test() {
        assert_eq!(linear(&[0, 5, -7, 100, 67, -23], &-7), Some(2));
        assert_eq!(linear(&[11, -25, 12, 85, -8], &6), None)
    }

    #[test]
    fn binary_test() {
        let fib = [1, 1, 2, 3, 5, 8, 13, 21];
        assert_eq!(binary(&fib, &5), Some(4));
        assert_eq!(binary(&fib, &21), Some(7));

        let primes = [1, 2, 3, 5, 7, 11, 13, 17];
        assert_eq!(binary(&primes, &8), None);
        assert_eq!(binary(&primes, &0), None);
        assert_eq!(binary(&primes, &18), None);
    }

    #[test]
    fn binary_first_test() {
        assert_eq!(binary(&[1, 1, 2, 3], &1), Some(1));
        assert_eq!(binary_first(&[1, 1, 2, 3], &1), Some(0));
    }
}
