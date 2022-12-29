mod parser;
pub mod tree;
use std::time::SystemTime;

fn main() {
    let arena = parser::parse();
    let start_p1 = SystemTime::now();
    part1(&arena);
    let end_p1 = SystemTime::now();
    part2(&arena);
    let end_p2 = SystemTime::now();

    let diff1 = end_p1.duration_since(start_p1).unwrap().as_micros();
    let diff2 = end_p2.duration_since(end_p1).unwrap().as_micros();

    println!("P1 Time: {}µs", diff1);
    println!("P2 Time: {}µs", diff2);
}

fn part1(arena: &tree::ArenaTree) {
    let v = arena.under100k();
    let sum: u32 = v.iter().sum();

    println!("Sum of spaces of directories under 100k: {}", sum);
}

fn part2(arena: &tree::ArenaTree) {
    let needed = 30000000 - (70000000 - arena.total_size());
    let mut v = arena.all_sizes();
    v.sort();
    for i in v {
        if i > needed {
            println!("Minimum deleted space: {}", i);
            break;
        }
    }
}
