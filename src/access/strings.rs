//!Operations on all of Rust's String types (&str, String, etc)
//! will go into the Strings library.

pub fn to_words(line: String) -> Vec<String>{
	line.split(' ').map(|c| {String::from(c)}).collect::<Vec<String>>()//TODO: Do tests, document, write examples
}