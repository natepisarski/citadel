//! Functions pertaining to the indexing of sets of data, as well as metadata
//! of such data.

use prelude;

/// Locate the indices of occurences of an element in a given list.
/// # Examples
/// ```
/// let spots_of_a = positions(&['a', 'a', 'b', 'a'], 'a')
///```
pub fn positions<T: Eq>(list: &[T], item: T) -> Vec<i32> {
    let mut collector: Vec<i32> = vec![];

    for counter in 0 .. list.len() {
        if list[counter] == item {
            collector.push(counter.clone() as i32);
        }
    }

    return collector;
}

/// Return everything before a match of the item.
/// # Examples
///
/// ```
/// use citadel::access::stat;
/// let one_two_three = stat::before(&[1, 2, 3, 4, 5], 4);
/// ```
pub fn before<T: Eq+Clone>(list: &[T], item: T) -> Vec<T> {
    prelude::filter_break(list, |c: &T| -> bool {*c != item})
}

/// Return everything after the match of an item.
/// Examples
/// ```
/// use citadel::access::stat;
/// let one_two_three = stat::after(&[4, 5, 1, 2, 3], 5)
///```
pub fn after<T: Eq+Clone>(list: &[T], item: T) -> Vec<T> {
    prelude::drop_break(list, |c: &T| -> bool {*c != item})
}
