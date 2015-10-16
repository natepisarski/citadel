//!Operations on all of Rust's String types (&str, String, etc)
//! will go into the Strings library.

// TODO: DOCUMENT / TEST
pub fn to_string_vec(list: Vec<&str>) -> Vec<String> {
	list.iter().map(|&c| {String::from(c)}).collect::<Vec<String>>()
}

pub fn to_words(line: String) -> Vec<String>{
	to_string_vec(line.split(' ').collect())
}