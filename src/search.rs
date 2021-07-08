use std::cmp::Ordering;

pub fn linear<T: PartialEq>(slice: &[T], value: T) -> Option<usize> {
    for (i, v) in slice.iter().enumerate() {
        if &value == v {
            return Some(i);
        }
    }

    None
}

pub fn binary<T: Ord>(slice: &[T], value: T) -> Option<usize> {
    let mid = slice.len() / 2;
    match value.cmp(&slice[mid]) {
        Ordering::Less if mid > 0 => binary(&slice[0..mid], value),
        Ordering::Equal => Some(mid),
        Ordering::Greater if mid < slice.len() - 1 => {
            match binary(&slice[(mid + 1)..slice.len()], value) {
                Some(x) => Some(x + mid + 1),
                None => None
            }
        },
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::linear;
    use super::binary;

    #[test]
    fn linear_test() {
        assert_eq!(linear(&[0, 5, -7, 100, 67, -23], -7), Some(2));
        assert_eq!(linear(&[11, -25, 12, 85, -8], 6), None)
    }

    #[test]
    fn binary_test() {
        let fib = [1, 1, 2, 3, 5, 8, 13, 21];
        assert_eq!(binary(&fib, 5), Some(4));
        assert_eq!(binary(&fib, 21), Some(7));
        
        let primes = [1, 2, 3, 5, 7, 11, 13, 17];
        assert_eq!(binary(&primes, 8), None);
        assert_eq!(binary(&primes, 0), None);
        assert_eq!(binary(&primes, 18), None);
    }
}
