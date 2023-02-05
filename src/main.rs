use lazy_static::lazy_static;
use regex::Regex;
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use clap::Parser;

/// Synopsoid: a markdown outline parser.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The path to the markdown file to parse.
    #[arg(short, long)]
    path: String,
}

fn main() {
    let args = Args::parse();
    parse_lines(args.path);
}

fn parse_line(line: &str) {
    lazy_static! {
        static ref RE_HEADER: Regex = Regex::new(r"^#+.*$").unwrap();
        static ref RE_H1: Regex = Regex::new(r"^#\s.*$").unwrap();
        static ref RE_H2: Regex = Regex::new(r"^##\s.*$").unwrap();
    }

    if RE_HEADER.is_match(line) {
        if RE_H1.is_match(line) {
            println!("H1: '{line}'");
        } else if RE_H2.is_match(line) {
            println!("H2: '{line}'");
        }
    }
}

fn parse_lines<P>(filename: P)
where
    P: AsRef<Path>,
{
    if let Ok(line_buffer) = read_lines(&filename) {
        for line in line_buffer {
            if let Ok(l) = line {
                parse_line(&l);
            } else {
                eprintln!("Failed to parse line");
            }
        }
    } else {
        eprintln!("Failed to open file '{}'", filename.as_ref().display());
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
