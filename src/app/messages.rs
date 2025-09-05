/// ----------------------------------------------------------------
/// IMPORTS
/// ----------------------------------------------------------------

use std::env;

use super::super::_core::strings::purify_string_length;

/// ----------------------------------------------------------------
/// METHODS
/// ----------------------------------------------------------------

#[allow(unused)]
pub fn welcome_screen() {
    let exe = env::current_exe().unwrap();
    let app_name: &str = exe.file_name().unwrap().to_str().unwrap();
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    const URL: &str = env!("CARGO_PKG_HOMEPAGE");
    let lines: Vec<String> = vec![
        format!("{app_name} \x1b[92;1mv{VERSION}\x1b[0m"),
        format!("{URL}"),
    ];
    display_bordered_message(lines);
}

/// ----------------------------------------------------------------
/// AUXILIARY METHODS
/// ----------------------------------------------------------------

fn display_bordered_message(lines: Vec<String>) {
    // determine padding
    let n = lines.iter().map(purify_string_length).max().unwrap_or(0);
    let hspace = " ".repeat(n + 2);
    let hbar = "\u{2500}".repeat(n + 2);

    println!("\u{250C}{hbar}\u{2510}");
    println!("\u{2502}{hspace}\u{2502}");
    let _: Vec<_> = lines.iter().map(|line| {
        let k = purify_string_length(line);
        let pad = " ".repeat(n - k);
        println!("\u{2502} {line}{pad} \u{2502}");
    }).collect();
    println!("\u{2502}{hspace}\u{2502}");
    println!("\u{2514}{hbar}\u{2518}");
}
