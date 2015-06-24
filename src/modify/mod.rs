//! The Modify branch of citadel handles the manipulation
//! of lists and slices, including the transformation and
//! coercions of such lists.
#[macro_use]
pub mod coerce;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_coerce() { 
        assert_eq!(str_to!("foo", Vec<char>), vec!['f', 'o', 'o']);
    }
}
