mod ansi_string;
mod ansi_parser;
use std::io::{self, BufRead};
use ansi_string::AnsiString;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    for line in lines {
        let line = line.unwrap();
        let ansi_string = AnsiString::new(line);
        println!("{}", ansi_string.without_colors);
    }
}
