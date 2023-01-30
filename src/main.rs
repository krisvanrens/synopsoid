use std::{io::{self, BufRead}, path::Path, fs::File};

use clap::Parser;

/// Synopsoid: a markdown outline parser.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The path to the markdown file to parse.
    #[arg(short, long)]
    path: String
}

fn main() {
    let args = Args::parse();
    if let Ok(lines) = read_lines(&args.path) {
        for line in lines {
            if let Ok(l) = line {
                println!("{l}");
            } else {
                eprintln!("Failed to parse line");
            }
        }
    } else {
        eprintln!("Failed to open file '{}'", &args.path);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
