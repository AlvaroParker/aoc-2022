use itertools::Itertools;
use std::fs;
use std::time::SystemTime;
mod part1;
mod part2;

fn main() {
    let v = open();
    let start_p1 = SystemTime::now();
    part1::part1(&v);
    let end_p1 = SystemTime::now();
    part2::part2(&v);
    let end_p2 = SystemTime::now();
    let diff1 = end_p1.duration_since(start_p1).unwrap();
    let diff2 = end_p2.duration_since(end_p1).unwrap();
    println!("Execution time P1: {}µs", diff1.as_micros());
    println!("Execution time P2: {}µs", diff2.as_micros());
}

fn open() -> Vec<Vec<(i32, i32)>> {
    let content = match fs::read_to_string("input") {
        Ok(s) => s,
        Err(_) => {
            panic!("Error while reading file.")
        }
    };
    let s = content
        .split_whitespace()
        .map(|s| {
            s.split(",")
                .map(|s| {
                    s.to_string()
                        .split("-")
                        .map(|n| n.parse::<i32>().unwrap())
                        .collect_tuple::<(i32, i32)>()
                        .unwrap()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    s
}
