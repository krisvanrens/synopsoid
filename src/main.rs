use colored::Colorize;
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
    sync::atomic::{AtomicBool, Ordering},
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

#[derive(Debug, PartialEq)]
enum Heading {
    H1(String),
    H2(String),
}

type Outline = Vec<Heading>;

trait Printable {
    fn print(&self);
}

// Example printed layout:
//
//   ⇒ Heading 1
//     ↳ Heading 2
//
impl Printable for Heading {
    fn print(&self) {
        match self {
            Heading::H1(title) => {
                static FIRST_PRINT: AtomicBool = AtomicBool::new(true);
                let prefix = if !FIRST_PRINT.load(Ordering::Relaxed) {
                    "\n"
                } else {
                    FIRST_PRINT.store(false, Ordering::SeqCst);
                    ""
                };
                println!("{}\u{21d2} {}", prefix, title.to_string().bold())
            }
            Heading::H2(title) => println!("  \u{21b3} {title}"),
        }
    }
}

fn main() {
    let args = Args::parse();
    let synopsis = parse_lines(args.path);

    for heading in synopsis {
        heading.print();
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
            static ref RE_HEADING: Regex = Regex::new(r"^#{1,2}\s.*$").unwrap();
            static ref RE_H1: Regex = Regex::new(r"^#\s(.*)$").unwrap();
            static ref RE_H2: Regex = Regex::new(r"^##\s(.*)$").unwrap();
        }

        if RE_HEADING.is_match(line) {
            let get_title = |re: &Regex| re.captures(line).unwrap()[1].to_string();

            return if RE_H1.is_match(line) {
                Some(Heading::H1(get_title(&RE_H1)))
            } else if RE_H2.is_match(line) {
                Some(Heading::H2(get_title(&RE_H2)))
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

    result.dedup();
    result
}
