use lazy_static::lazy_static;
use regex::Regex;
use std::fs::read_to_string;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
pub enum PuzzleError {
    MissingFile,
    ParseError { line: usize, err: ParseLineError },
}

#[derive(Debug)]
pub enum ParseLineError {
    IntParse(ParseIntError),
    BadLength(String),
}

impl From<ParseIntError> for ParseLineError {
    fn from(value: ParseIntError) -> Self {
        ParseLineError::IntParse(value)
    }
}

fn parse_line(str: &str) -> Result<(u32, u32, u32, u32), ParseLineError> {
    // Useless here to use lazy_static since this function is always called, but it's good practice.
    lazy_static! {
        static ref RE: Regex = Regex::from_str(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    }

    if let Some((_, [a, b, c, d])) = RE.captures(str).map(|c| c.extract()) {
        Ok((a.parse()?, b.parse()?, c.parse()?, d.parse()?))
    } else {
        Err(ParseLineError::BadLength(str.to_owned()))
    }
}

fn main() -> Result<(), PuzzleError> {
    let mut total_contained = 0;
    let text = read_to_string("./assets/day4.txt").or(Err(PuzzleError::MissingFile))?;

    for (i, line) in text.lines().enumerate() {
        let x = parse_line(line).or_else(|err| Err(PuzzleError::ParseError { line: i, err }))?;
        let (start1, end1, start2, end2) = x;
        if (start1 <= start2 && end2 <= end1) || (start2 <= start1 && end1 <= end2) {
            total_contained += 1;
        }
    }

    println!("Ranges fully contained in each other {}", total_contained);
    Ok(())
}
