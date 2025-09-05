/// ----------------------------------------------------------------
/// IMPORTS
/// ----------------------------------------------------------------

use strip_ansi_escapes::strip;

/// ----------------------------------------------------------------
/// METHODS
/// ----------------------------------------------------------------

pub fn greet(name: &str) {
    println!("Hello, {}!", name);
}

#[allow(unused)]
/// Strips potential ANSII characters
pub fn purify_string(text: &String) -> String {
    String::from_utf8(strip(text)).unwrap_or(text.clone())
}

#[allow(unused)]
/// Computes length of purified string
pub fn purify_string_length(text: &String) -> usize {
    purify_string(text).len()
}
