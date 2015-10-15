//! Modules relating to the retrieval of information of slices / lists of information.
//! Functions and structures which provide information about the contents of a list, or
//! obtaining specific parts of a sequence go here. In general, if the list is being 
//! modified, it belongs in the modify wing, not access.

pub mod stat;
pub mod encompassing;
pub mod strings;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stat() {
        
        /* after */
       assert_eq!(stat::after(&[1, 2, 3, 4], 2), vec![3, 4]);

        /* before */
        assert_eq!(stat::before(&[1, 2, 3, 4], 2), vec![1]);
        
        /* positions */
        assert_eq!(stat::positions(&[1, 2, 1, 2], 1), vec![0 as i32, 2 as i32]);

        /* between */
        assert_eq!(stat::between(&[1, 2, 1, 1, 2, 1], (2, 2)), vec![1, 1]);
    }

    #[test]
    fn test_encompassing() {
        
        /* encompassed_by */
        assert_eq!(encompassing::encompassed_by(
            &['a', '{', 'b', '{', 'c', '}', '}', 'a', '{', 'd', '}', 'e'], ('{', '}')),
                   vec![vec!['b', '{', 'c', '}'], vec!['d']]);
    }
}
