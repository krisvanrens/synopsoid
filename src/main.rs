use clap::Parser;
use std::fs;
use synopsoid::lib::*;

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
