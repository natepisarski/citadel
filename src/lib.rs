pub mod access;
pub mod prelude;

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_prelude() {

        /* collect_as_vec */
        assert_eq!(prelude::collect_as_vector(&[1, 2, 3, 4, 5], (1, 3)), vec![2, 3, 4]);
        assert_eq!(prelude::collect_as_vector(&[1, 2, 3, 4, 5], (3, 4)), vec![4, 5]);

        /* drop_break */
        assert_eq!(prelude::drop_break(&[1, 2, 3], |x: &usize| -> bool {*x != 2}), vec![3]);

        /* filter_break */
        assert_eq!(prelude::filter_break(&[1, 2, 3], |x: &usize| -> bool {*x != 2}), vec![1]);
    }
}
