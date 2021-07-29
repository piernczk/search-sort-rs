//! Implementations of sorting algorithms.

use std::cmp::Ordering;

/// Checks if a slice is sorted.
pub fn test<T: Ord>(slice: &[T]) -> bool {
    if slice.len() < 2 {
        return true;
    }

    let mut iter = slice.iter();
    let mut prev = iter.next().unwrap();
    let mut curr = iter.next();

    while curr != None {
        let value = curr.unwrap();
        if prev > value {
            return false;
        }

        prev = value;
        curr = iter.next();
    }

    true
}

/// Alias for [`test()`].
pub fn is_sorted<T: Ord>(slice: &[T]) -> bool {
    test(slice)
}

/// An implementation of bubble sort.
///
/// Checks for every element if the next element is greater than this and swaps
/// them if so. Then repeats the process until the list is sorted.
///
/// # Examples
/// ```
/// use search_sort::sort;
///
/// let mut slice = [1, 6, 3, -44, 11, 2];
/// sort::bubble(&mut slice);
/// assert_eq!(slice, [-44, 1, 2, 3, 6, 11]);
/// ```
pub fn bubble<T: Ord>(slice: &mut [T]) {
    let mut n = slice.len();
    while n > 1 {
        let mut newn = 0;

        for i in 1..n {
            if slice[i - 1] > slice[i] {
                slice.swap(i - 1, i);
                newn = i;
            }
        }

        n = newn;
    }
}

/// Part of quick sort algorithm.
///
/// Sets the pivot, places smaller elements before it and greater after it.
/// Returns the final position of the pivot.
///
/// This function is used in [`quick`] sort.
pub fn quick_partition<T: Ord>(slice: &mut [T]) -> usize {
    // 'the pivot' is the last element of the slice

    let n = slice.len();
    let mut lo = 0;
    let mut hi = n - 1;
    let mut pivot = n - 1;

    let mut equal = false;
    loop {
        // always increments the counter if they're equal
        if equal {
            lo += 1;
            equal = false;
        }

        // search for an element greater or equal to the pivot
        while slice[lo] < slice[pivot] {
            lo += 1;
        }

        // search for an element smaller or equal to the pivot
        while hi > 0 && slice[hi] > slice[pivot] {
            hi -= 1;
        }

        if lo >= hi {
            // the slice is sorted
            break;
        } else if slice[lo] == slice[hi] {
            equal = true;
        } else {
            if lo == pivot {
                pivot = hi;
            } else if hi == pivot {
                pivot = lo;
            }

            slice.swap(lo, hi);
        }
    }

    slice.swap(lo, pivot);
    lo
}

/// An implementation of quick sort.
///
/// Partitions the slice into two parts by [`quick_partition`], and invokes
/// itself until the list is sorted.
///
/// # Examples
/// ```
/// use search_sort::sort;
///
/// let mut slice = [5, 1, -5, 3, 9, 2, 19];
/// sort::quick(&mut slice);
/// assert_eq!(slice, [-5, 1, 2, 3, 5, 9, 19]);
/// ```
pub fn quick<T: Ord>(slice: &mut [T]) {
    if slice.len() > 1 {
        let partition = quick_partition(slice);
        quick(&mut slice[..partition]);
        quick(&mut slice[(partition + 1)..]);
    }
}

/// An implemetation of top-down (recursive) merge sort that uses only
/// half of the space.
///
/// Invokes itself on the two halves, copies the first half of the slice and
/// merges it into the original slice.
///
/// # Examples
/// ```
/// use search_sort::sort;
///
/// let mut slice = [4, -2, 7, 0, 11, -11, -10];
/// sort::merge(&mut slice);
/// assert_eq!(slice, [-11, -10, -2, 0, 4, 7, 11]);
/// ```
pub fn merge<T: Ord + Clone>(slice: &mut [T]) {
    if slice.len() > 1 {
        let mid = slice.len() / 2;

        // copy first part to a new slice
        let mut left = Vec::new();
        left.extend_from_slice(&slice[..mid]);
        let mut left = &mut left[..];

        merge(&mut left);
        merge(&mut slice[mid..]);

        // merge the two parts
        let mut i = 0;
        let mut j = 0;
        loop {
            let midj = mid + j;

            if i == mid {
                // the slice is sorted, as the first part has been merged
                break;
            } else if midj == slice.len() {
                // only the second half has been merged, so clone the remainging
                // elements
                slice[(mid + i)..].clone_from_slice(&left[i..]);
                break;
            }

            let ij = i + j;

            match left[i].cmp(&slice[midj]) {
                Ordering::Less => {
                    slice[ij] = left[i].clone();
                    i += 1;
                }
                Ordering::Equal => {
                    // insert the two elements one by one, since they are equal

                    let e = left[i].clone();

                    slice[ij] = e.clone();
                    slice[ij + 1] = e;

                    i += 1;
                    j += 1;
                }
                Ordering::Greater => {
                    slice[i + j] = slice[midj].clone();
                    j += 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::bubble;
    use super::merge;
    use super::quick;
    use super::test;

    #[test]
    fn test_test() {
        // lol
        assert!(test(&[0, 3, 4, 10, 12]));
        assert!(!test(&[6, 2, 1, 10, -2]));
    }

    #[test]
    fn bubble_test() {
        let mut data = [4, 2, 1, 8, 7, 9, -11];
        bubble(&mut data);
        assert_eq!(data, [-11, 1, 2, 4, 7, 8, 9]);
    }

    #[test]
    fn quick_test() {
        let mut data = [6, 7, 3, 5, 4, -12];
        quick(&mut data);
        assert_eq!(data, [-12, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn merge_test() {
        let mut data1 = [6, 1, 2, 99, -1, 13, 7, 1];
        let mut data2 = [5, 7, 11, -1, 2, 3];
        let mut data3 = [11, 16, 20, 12, 13, 15];

        merge(&mut data1);
        merge(&mut data2);
        merge(&mut data3);

        assert_eq!(data1, [-1, 1, 1, 2, 6, 7, 13, 99]);
        assert_eq!(data2, [-1, 2, 3, 5, 7, 11]);
        assert_eq!(data3, [11, 12, 13, 15, 16, 20]);
    }
}
