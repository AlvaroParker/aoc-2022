use std::{collections::HashSet, fs};

fn open() -> Vec<String> {
    let content = match fs::read_to_string("input") {
        Ok(s) => s,
        Err(_) => {
            panic!("Error while reading file")
        }
    };
    content
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
}
fn compare(s1: &String, s2: &String, s3: &String) -> u32 {
    let h1 = s1.chars().collect::<HashSet<char>>();
    let h2 = s2.chars().collect::<HashSet<char>>();
    let h3 = s3.chars().collect::<HashSet<char>>();
    let mut counter = 0;
    for c in h1
        .intersection(&h2)
        .collect::<String>()
        .chars()
        .collect::<HashSet<char>>()
        .intersection(&h3)
    {
        if c.is_uppercase() {
            counter += *c as u32 - 38;
        } else {
            counter += *c as u32 - 96;
        }
    }
    counter
}

pub fn part2() {
    let mut counter = 0;
    let content = open();
    for i in 0..content.len() / 3 {
        counter += compare(
            content.get(3 * i).unwrap(),
            content.get(3 * i + 1).unwrap(),
            content.get(3 * i + 2).unwrap(),
        );
    }
    println!("Final count p2: {}", counter);
}
