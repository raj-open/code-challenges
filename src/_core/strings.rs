// ----------------------------------------------------------------
// IMPORTS
// ----------------------------------------------------------------

use itertools::Itertools;
use regex::Regex;
use strip_ansi_escapes::strip;

// ----------------------------------------------------------------
// METHODS
// ----------------------------------------------------------------

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
    // remove ansii characters
    let text = purify_string(text);
    // since unicode chars can have length > 1, convert to chars
    let chars = text.chars();

    chars.count()
}

#[allow(unused)]
/// Performs a horizontal string join for multline strings
pub fn join_multiline_strings(
    blocks: &Vec<String>,
    sep_block: Option<&String>,
    sep_single: &str,
) -> String {
    // split blocks into lines and pad to ensure consistency of widths
    let blocks: Vec<Vec<String>> = blocks.iter().map(pad_widths).collect();

    // pad to ensure consistencey of heights
    let height = blocks.iter().map(|lines| lines.len()).max().unwrap_or(0);
    let blocks: Vec<Vec<String>> = blocks.iter().map(|lines| pad_height(lines, height)).collect();

    // determine a separator
    let sep_block0 = ([sep_single].repeat(height)).join("\n");
    let sep_block = sep_block.map_or_else(|| sep_block0, |x| x.clone());
    let sep_block = pad_widths(&sep_block);
    let sep_block = pad_height(&sep_block, height);

    // join blocks line-wise

    (0..height)
        .map(|i| {
            let empty = "".to_string();
            let sep = sep_block.get(i).unwrap_or(&empty);
            blocks.iter().map(|lines| lines.get(i).unwrap_or(&empty)).join(sep)
        })
        .join("\n")
}

/// Splits a string into lines and pads to ensure uniformity of widths
fn pad_widths(text: &String) -> Vec<String> {
    let re = Regex::new(r"\r?\n").unwrap();
    let lines: Vec<String> = re.split(text).map(|x| x.to_string()).collect();
    let width = lines.iter().map(purify_string_length).max().unwrap_or(0);
    let lines: Vec<String> = lines
        .iter()
        .map(|x| {
            let n = purify_string_length(x);
            let space = " ".repeat(width - n);
            format!("{x}{space}")
        })
        .collect();
    lines
}

/// Splits a string into lines and pads to ensure uniformity of widths
fn pad_height(lines: &Vec<String>, height: usize) -> Vec<String> {
    let n = lines.len();
    if n >= height {
        return lines.clone();
    }
    let line = lines.first().cloned().unwrap_or("".to_string());
    let width = purify_string_length(&line);
    let space = " ".repeat(width);
    let pad = vec![space.clone(); height - n];

    [lines.to_owned(), pad].concat()
}
