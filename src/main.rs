use regex::Regex;
use std::fs;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Debug)]
struct Hanzi {
    ids: Vec<u32>,
    simplified: String,
    traditional: String,
}

fn str_to_u32(s: &str) -> Option<u32> {
    let mut r = 0;
    for c in s.chars() {
        match c.to_digit(10) {
            Some(digit) => r = (r * 10) + digit as u32,
            None => return None,
        }
    }
    Some(r)
}

impl FromStr for Hanzi {
    type Err = std::str::Utf8Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut hanzi = Hanzi {
            simplified: "".to_owned(),
            traditional: "".to_owned(),
            ids: vec![],
        };

        let re = Regex::new(r"(\p{L}+|\d+)").unwrap();

        for group in re.find_iter(s).map(|m| m.as_str()) {
            match str_to_u32(group) {
                Some(n) => hanzi.ids.push(n),
                None if hanzi.simplified == "" => hanzi.simplified = group.to_string(),
                None => hanzi.traditional = group.to_string(),
            }
        }
        if hanzi.traditional == "" {
            hanzi.traditional = hanzi.simplified.clone();
        }
        Ok(hanzi)
    }
}

fn main() {
    let file = fs::File::open("./data/cedict.idx").unwrap();
    let buffered = BufReader::new(file);

    let mut result: Vec<Hanzi> = vec![];

    for line in buffered.lines() {
        match Hanzi::from_str(&line.unwrap()) {
            Ok(hanzi) => result.push(hanzi),
            Err(_) => {}
        }
    }
    println!("Ok");
    println!("{:?}", result);
}
