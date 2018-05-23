extern crate ucd;
use std::io::{stdin, stdout, Write, Result};
use std::env::args;

use ucd::{UnicodeCategory, Codepoint};

const DEFUALT_TERM_WIDTH: usize = 120;

fn main() -> Result<()> {

    let mut args = args();
    args.next();

    let width =
        args.next().and_then(|x| x.trim().parse::<usize>().ok()).unwrap_or(DEFUALT_TERM_WIDTH);

    let stdin = stdin();
    let mut stdout = stdout();
    let mut x = 0;
    let mut last_space = false;
    let mut last_indent = None;

    let mut current_line = String::new();

    loop {
        let mut line = String::new();
        stdin.read_line(&mut line)?;
        if line == "" {
            break;
        }
        line = line.trim_right().into();
        line.push(' ');
        let mut chars = line.chars();
        let mut indent = 0;
        while let Some(next) = chars.as_str().chars().next() {
            if next.category() == UnicodeCategory::SpaceSeparator {
                chars.next();
                indent += 1;
            } else {
                break;
            }
        }
        if indent != last_indent.unwrap_or(indent) {
            println!("{}", current_line.trim_right());
            current_line = String::new();
            x = 0;
        }
        while x < indent {
            current_line.push(' ');
            x += 1;
        }

        while let Some(word_start) = chars.next() {
            // Find word
            let mut word = word_start.to_string();
            let cat = word_start.lowercase_simple().category();
            while let Some(next) = chars.as_str().chars().next() {
                if next.lowercase_simple().category() == cat {
                    word.push(next);
                    chars.next();
                }
                else {
                    break;
                }
            }
            if x + word.len() >= width {
                println!("{}", current_line.trim_right());
                current_line = String::new();
                for _ in 0..indent {
                    current_line.push(' ');
                }
                last_space = true;
                x = indent;
            }
            if cat == UnicodeCategory::SpaceSeparator && last_space {
                continue;
            }
            x += word.len();
            current_line.push_str(&word);
            last_space = cat == UnicodeCategory::SpaceSeparator;
        }
        last_indent = Some(indent);
    }
    print!("{}", current_line);
    stdout.flush()?;
    Ok(())
}
