use colored::Colorize;
use lazy_static::lazy_static;
use regex::Regex;
use serde::Serialize;
use std::{
    fmt,
    fs::{self, File},
    io::{self, BufRead},
    path::Path,
};

use clap::Parser;

/// Synopsoid: a markdown outline parser.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The path to the input markdown file to parse.
    #[arg(short, long)]
    path: String,

    /// The path to an (optional) output JSON file. Providing this will disable printing.
    #[arg(short, long, default_value = "")]
    output: String,
}

#[derive(Debug, PartialEq, Serialize)]
enum Heading {
    H1(String),
    H2(String),
}

#[derive(Debug, Serialize)]
struct Outline {
    headings: Vec<Heading>,
}

impl Outline {
    fn new() -> Self {
        Self {
            headings: Vec::new(),
        }
    }
}

// Example printed layout:
//
//   ⇒ Heading 1
//     ↳ Heading 2
//
impl fmt::Display for Outline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first_h1 = true;
        for heading in &self.headings {
            match heading {
                Heading::H1(title) => {
                    let prefix = if first_h1 { "" } else { "\n" };
                    first_h1 = false;
                    writeln!(f, "{}\u{21d2} {}", prefix, title.to_string().bold())?
                }
                Heading::H2(title) => writeln!(f, "  \u{21b3} {title}")?,
            }
        }
        Ok(())
    }
}

fn main() {
    let args = Args::parse();
    let synopsis = parse_lines(args.path);

    if args.output.is_empty() {
        print!("{synopsis}");
    } else {
        fs::write(
            &args.output,
            serde_json::to_string_pretty(&synopsis).unwrap(),
        )
        .unwrap_or_else(|_| panic!("unable to write output file '{}'", &args.output));
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

    fn clean_title(title: &str) -> String {
        lazy_static! {
            static ref RE_HTML: Regex = Regex::new(r"(<[/]*[a-z0-9\-]+>)").unwrap();
            static ref RE_HTML_BR: Regex = Regex::new(r"(<br/>)").unwrap();
            static ref RE_MD_QUOTE: Regex = Regex::new(r"([`*])").unwrap();
            static ref RE_MD_STYLE: Regex = Regex::new(r"(\{\.[a-z]{2,3}\})").unwrap();
        }

        let mut result: String;

        result = RE_HTML.replace_all(title, "").to_string();
        result = RE_HTML_BR.replace_all(&result, " ").to_string();
        result = RE_MD_QUOTE.replace_all(&result, "").to_string();
        result = RE_MD_STYLE.replace_all(&result, "").to_string();
        result
    }

    fn parse_line(line: &str) -> Option<Heading> {
        lazy_static! {
            static ref RE_HEADING: Regex = Regex::new(r"^#{1,2}\s.*$").unwrap();
            static ref RE_H1: Regex = Regex::new(r"^#\s(.*)$").unwrap();
            static ref RE_H2: Regex = Regex::new(r"^##\s(.*)$").unwrap();
        }

        if RE_HEADING.is_match(line) {
            let get_title = |re: &Regex| clean_title(&re.captures(line).unwrap()[1]);

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
                    result.headings.push(heading);
                }
            } else {
                eprintln!("Failed to parse line");
            }
        }
    } else {
        eprintln!("Failed to open file '{}'", filename.as_ref().display());
    }

    result.headings.dedup();
    result
}
