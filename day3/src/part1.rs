use std::{collections::HashSet, fs};
fn open() -> Vec<String> {
    let content = match fs::read_to_string("input") {
        Ok(a) => a,
        Err(_) => {
            panic!("Error while reading file.")
        }
    };
    content
        .split_whitespace()
        .map(|s| String::from(s))
        .collect::<Vec<String>>()
}

fn compare(s: String) -> u32 {
    let mut counter = 0;
    let mid = s.len() / 2;
    let (s1, s2) = s.split_at(mid);
    let h1 = s1.chars().collect::<HashSet<char>>();
    let h2 = s2.chars().collect::<HashSet<char>>();
    for c in h1.intersection(&h2) {
        if c.is_uppercase() {
            counter += *c as u32 - 38;
        } else {
            counter += *c as u32 - 96;
        }
    }
    counter
}

pub fn part1() {
    let v = open();
    let mut counter = 0;
    for s in v {
        counter += compare(s);
    }
    println!("Total count: {}", counter);
}
