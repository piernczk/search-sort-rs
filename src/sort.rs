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

#[cfg(test)]
mod tests {
    use super::bubble;

    #[test]
    fn bubble_test() {
        let mut data = [4, 2, 1, 8, 7, 9, -11];
        bubble(&mut data);
        assert_eq!(data, [-11, 1, 2, 4, 7, 8, 9]);
    }
}
