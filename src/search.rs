//! Implementations of searching algorithms.

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
/// the slice before the found element. Returns the position of the last
/// (first in the slice) equal element.
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

/// An implementation of jump search with custom `step`.
///
/// Jumps over a sorted slice by fixed steps, until it finds the largest
/// element, smaller than the value. Then invokes [linear] search from this
/// element to the next step.
///
/// It's usually slower than `binary` search, except when the value is expected
/// to be on the beggining of the slice.
/// 
/// See also [`jump`] function.
pub fn jump_step<T: Ord>(slice: &[T], value: &T, step: usize) -> Option<usize> {
    if step == 1 {
        return linear(slice, value);
    } else if step == 0 {
        // it would be stuck on the first element
        if &slice[0] == value {
            return Some(0);
        } else {
            return None;
        }
    }

    let mut iter = slice.iter();
    let mut pos: usize = 0;

    // if found larger than the value
    let mut found = false;

    for i in 0..(slice.len() / step) {
        match value.cmp(iter.next().unwrap()) {
            Ordering::Less => {
                if i == 0 {
                    // smaller than every element
                    return None;
                }

                found = true;
                break;
            }
            Ordering::Equal => return Some(i * step),
            Ordering::Greater => {
                pos = i * step;
                if i >= slice.len() {
                    // larger than every element
                    return None;
                }
            }
        }

        // Iterator::nth(...) adds 1 automatically; hence step - 1
        // additionally, Iterator::next() is invoked above; thus step - 2
        iter.nth(step - 2).unwrap();
    }

    let mut end = pos + step;
    if !found {
        // if did not found, then check the rest of the slice, since the loop
        // may not have done it; plus pos + step could be higher than len - 1
        end = slice.len()
    }

    // no need to check the element on pos
    linear(&slice[(pos + 1)..end], value).map(|x| x + pos + 1)
}

/// An implementation of jump search with optimal `step`.
/// 
/// Invokes [`jump_step`] search with square root of the length of the slice.
///
/// # Examples
///
/// ```
/// use search_sort::search;
///
/// let slice = [1, 5, 7, 15, 31, 32, 45];
/// assert_eq!(search::jump(&slice, &15), Some(3));
/// ```
pub fn jump<T: Ord>(slice: &[T], value: &T) -> Option<usize> {
    jump_step(slice, value, (slice.len() as f64).sqrt() as usize)
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
