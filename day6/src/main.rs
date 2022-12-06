use std::time::SystemTime;
use std::{collections::HashSet, fs};

fn main() {
    let content = open();
    let start_p1 = SystemTime::now();
    part1(&content);
    let end_p1 = SystemTime::now();
    part2(&content);
    let end_p2 = SystemTime::now();

    let diff_p1 = end_p1.duration_since(start_p1).unwrap().as_micros();
    let diff_p2 = end_p2.duration_since(end_p1).unwrap().as_micros();

    println!("Time P1: {}µs", diff_p1);
    println!("Time P2: {}µs", diff_p2);
}

fn open() -> String {
    let content = match fs::read_to_string("input") {
        Ok(s) => s,
        Err(e) => {
            panic!("Panic when reading file: {}", e)
        }
    };
    content
}

fn part1(content: &String) {
    for i in 4..content.len() {
        let s = &content[i - 4..i];
        if is_unique(s) {
            println!("Start of packet marker detected at: {}", i);
            break;
        }
    }
}

fn is_unique(s: &str) -> bool {
    let mut uniq = HashSet::new();
    s.chars().all(move |x| uniq.insert(x))
}

fn part2(content: &String) {
    for i in 14..content.len() {
        let s = &content[i - 14..i];
        if is_unique(s) {
            println!("Start of message marker detected at: {}", i);
            break;
        }
    }
}
