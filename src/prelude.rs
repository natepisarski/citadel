//! Functions which are universal to citadel go in prelude, so that they may
//! be included in further submodules without causing cross-depdency between 
//! branches of equal order.

/// Collects a slice (determined by the supplied bounds) as a vector, cloning
/// the values.
/// # Examples
/// ```
/// use citadel::prelude;
/// let one_two_three: Vec<usize> = prelude::collect_as_vector(&[0, 0, 1, 2, 3, 0, 0], (2 as usize, 4 as usize));
/// ```
pub fn collect_as_vector<T: Clone>(list: &[T], (bound1, bound2): (usize, usize)) -> Vec<T> {
    let mut collector: Vec<T> = vec![];

    for index in bound1 .. bound2 + 1 {
        collector.push(list[index].clone());
    }

    collector
}

/// Filters a list, ceasing to collect once a predicate returns FALSE.
/// # Examples
/// ```
/// use citadel::prelude;
/// let one_two_three = prelude::filter_break(&[1, 2, 3, 4], |x: &usize| -> bool {*x != 4});
/// ```
pub fn filter_break<T: Clone, F>(list: &[T], pred: F) -> Vec<T> 
    where F: Fn(&T) -> bool {
    let mut collector: Vec<T> = vec![];

    for item in list {
        if ! pred(item) {
            return collector.clone();
        }
        collector.push(item.clone());
    }
        return collector.clone();
}

/// Iterate over a list of items, returning the remaining items after a
/// predicate yields FALSE.
/// # Examples
/// ```
/// use citadel::prelude;
/// let one_two_three = prelude::drop_break(&[4, 5, 6, 1, 2, 3], |x: &usize| -> bool {*x != 6});
/// ```
pub fn drop_break<T: Clone, F>(list: &[T], pred: F) -> Vec<T> // Clumsy function
    where F: Fn(&T) -> bool {
        for item in 0 .. list.len() {
            if ! pred(&list[item]) {
                return collect_as_vector(list, (item + 1, list.len() - 1));
            }
        }
        vec![]
}
