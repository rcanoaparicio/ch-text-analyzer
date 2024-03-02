use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};

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

fn get_ids(s: &str) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();
    let parts = s.split(',');
    for part in parts {
        match str_to_u32(part) {
            Some(n) => result.push(n),
            None => {}
        }
    }
    result
}

fn get_word(s: &str) -> String {
    s.split(',').next().unwrap().to_string()
}

fn main() {
    let file = fs::File::open("./data/cedict.idx").unwrap();
    let buffered = BufReader::new(file);

    let mut result: HashMap<String, Vec<u32>> = HashMap::new();

    for line in buffered.lines() {
        match line {
            Err(_) => {}
            Ok(l) => {
                let word = get_word(&l);
                let ids = get_ids(&l);
                result.insert(word, ids);
            }
        }
    }
    println!("Ok");
    println!("你好 {:?}", result.get("你好"))
}
