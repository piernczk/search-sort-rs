pub fn linear<T: PartialEq>(slice: &[T], value: T) -> Option<usize> {
    for (i, v) in slice.iter().enumerate() {
        if &value == v {
            return Some(i);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::linear;

    #[test]
    fn linear_test() {
        assert_eq!(linear(&[0, 5, -7, 100, 67, -23], -7), Some(2));
        assert_eq!(linear(&[11, -25, 12, 85, -8], 6), None)
    }
}
