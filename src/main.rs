extern crate ucd;
use std::io::{stdin, stdout, Write, Result};

use ucd::{UnicodeCategory, Codepoint};

const TERM_WIDTH: usize = 31;

fn main() -> Result<()> {
    let stdin = stdin();
    let mut stdout = stdout();
    let mut x = 0;
    let mut last_space = false;
    let mut indent = 0;
    loop {
        let mut line = String::new();
        stdin.read_line(&mut line)?;
        if line == "" {
            break;
        }
        line = line.trim_right().into();
        line.push(' ');
        let mut chars = line.chars();
        let last_indent = indent;
        indent = 0;
        while let Some(next) = chars.as_str().chars().next() {
            if next.category() == UnicodeCategory::SpaceSeparator {
                chars.next();
                indent += 1;
            } else {
                break;
            }
        }
        if last_indent != indent {
            println!();
            x = 0;
        }
        while x < indent {
            print!(" ");
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
            if x + word.len() >= TERM_WIDTH {
                println!();
                for _ in 0..indent {
                    print!(" ");
                }
                last_space = true;
                x = indent;
            }
            if cat == UnicodeCategory::SpaceSeparator && last_space {
                continue;
            }
            x += word.len();
            print!("{}", word);
            last_space = cat == UnicodeCategory::SpaceSeparator;
        }
    }
    stdout.flush()?;
    Ok(())
}
