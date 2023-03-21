mod filereader;

pub mod lib {
    use crate::filereader::*;
    use colored::Colorize;
    use lazy_static::lazy_static;
    use regex::Regex;
    use serde::Serialize;
    use std::{fmt, path::Path};

    #[derive(Debug, PartialEq, Serialize)]
    pub enum Heading {
        H1(String),
        H2(String),
    }

    #[derive(Debug, Serialize)]
    pub struct Outline {
        pub headings: Vec<Heading>,
    }

    impl Outline {
        pub fn new() -> Self {
            Self {
                headings: Vec::new(),
            }
        }
    }

    impl Default for Outline {
        fn default() -> Self {
            Self::new()
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

    pub fn parse_lines<P>(filename: P) -> Outline
    where
        P: AsRef<Path>,
    {
        let mut result = Outline::new();

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

        if let Ok(file) = FileReader::try_new(&filename) {
            for line in file {
                if let Some(heading) = parse_line(&line) {
                    result.headings.push(heading);
                }
            }
        } else {
            eprintln!("failed to open file '{}'", filename.as_ref().display());
        }

        result.headings.dedup();
        result
    }
}
