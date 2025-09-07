/// ----------------------------------------------------------------
/// IMPORTS
/// ----------------------------------------------------------------

use itertools::Itertools;
use regex::Regex;
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
    // remove ansii characters
    let text = purify_string(text);
    // since unicode chars can have length > 1, convert to chars
    let chars = text.chars();
    let n = chars.count();
    return n;
}

#[allow(unused)]
/// Performs a horizontal string join for multline strings
pub fn join_multiline_strings(
    blocks: &Vec<String>,
    sep: &str,
) -> String {
    // split blocks into lines and pad to ensure consistency of widths
    let re = Regex::new(r"\r?\n").unwrap();
    let blocks: Vec<Vec<String>> = blocks
        .iter()
        .map(|x| {
            let lines: Vec<String> = re.split(x).map(|x| x.to_string()).collect();
            let width = lines.iter().map(purify_string_length).max().unwrap_or(0);
            let lines: Vec<String> = lines
                .iter()
                .map(|x| {
                    let n = purify_string_length(x);
                    let space = " ".repeat(width - n);
                    return format!("{x}{space}");
                })
                .collect();
            return lines;
        })
        .collect();

    // pad to ensure consistencey of heights
    let height = blocks.iter().map(|lines| lines.len()).max().unwrap_or(0);
    let blocks: Vec<Vec<String>> = blocks
        .iter()
        .map(|lines| {
            let n = lines.len();
            let line = lines.get(0).map(|x| x.clone()).unwrap_or("".to_string());
            let width = purify_string_length(&line);
            let space = " ".repeat(width);
            let pad = vec![space.clone(); height - n];
            let lines = [lines.to_owned(), pad].concat();
            return lines;
        })
        .collect();

    let result = (0.. height)
        .map(|i| {
            blocks
            .iter()
            .map(|lines| lines.get(i).unwrap())
            .join(sep)
        })
        .join("\n");

    return result;
}
