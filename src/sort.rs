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

    loop {
        // search for an element greater or equal to the pivot
        while slice[lo] < slice[n - 1] {
            lo += 1;
        }

        // search for an element smaller or equal to the pivot
        while hi > 0 && slice[hi] > slice[n - 1] {
            hi -= 1;
        }

        if lo >= hi {
            // the slice is sorted
            break;
        } else {
            slice.swap(lo, hi);
        }
    }

    slice.swap(lo, n - 1);
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

#[cfg(test)]
mod tests {
    use super::bubble;
    use super::quick;

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
}
