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

#[derive(Debug)]
enum Heading {
    H1(String),
    H2(String),
}

type Outline = Vec<Heading>;

fn main() {
    let args = Args::parse();
    let synopsis = parse_lines(args.path);

    // XXX
    println!("Synopsis size: {} items", synopsis.len());
    for line in synopsis {
        println!("{line:?}");
    }
}

fn parse_lines<P>(filename: P) -> Outline
where
    P: AsRef<Path>,
{
    let mut result = Outline::new();

    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    fn parse_line(line: &str) -> Option<Heading> {
        lazy_static! {
            static ref RE_HEADER: Regex = Regex::new(r"^#+.*$").unwrap();
            static ref RE_H1: Regex = Regex::new(r"^#\s.*$").unwrap();
            static ref RE_H2: Regex = Regex::new(r"^##\s.*$").unwrap();
        }

        if RE_HEADER.is_match(line) {
            return if RE_H1.is_match(line) {
                Some(Heading::H1("H1".to_string()))
            } else if RE_H2.is_match(line) {
                Some(Heading::H2("H2".to_string()))
            } else {
                None
            };
        }

        None
    }

    if let Ok(line_buffer) = read_lines(&filename) {
        for line in line_buffer {
            if let Ok(l) = line {
                if let Some(heading) = parse_line(&l) {
                    result.push(heading);
                }
            } else {
                eprintln!("Failed to parse line");
            }
        }
    } else {
        eprintln!("Failed to open file '{}'", filename.as_ref().display());
    }

    result
}
