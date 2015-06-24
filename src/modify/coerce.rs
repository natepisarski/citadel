//! Functions and macros for coercing lists from one type to another.
//! functions which collect just certain subsequences as a type 
//! (i.e `citadel::prelude::collect_as_vector) do not apply.

#[macro_export]
macro_rules! str_to {
    ($string:expr, $typ:ty) => ($string.to_string().chars().collect::<$typ>());
}
