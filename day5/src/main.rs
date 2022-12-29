use std::time::SystemTime;
mod common;
mod part1;
mod part2;

fn main() {
    // Get content from "input" file
    let content = common::open();
    // Get the stack (as a raw String) from input
    let lines = content.get(0).unwrap();
    // Get the instructions (as a raw String) from input
    let inst = content.get(1).unwrap();

    // Parse the stack to an array of vectors, each vector represents
    // a column, so stacks_array[0] represent the first column and so on
    let mut stacks_array = common::stacks(lines);
    let mut stack_array_p1 = stacks_array.clone();
    // Parse instruction to a tuple with 3 i32 elements
    // first element -> move n
    // second element -> to k
    // third element -> from z
    let instruct = common::instructions(inst);
    // First part
    let start_p1 = SystemTime::now();
    let i = 32;
    part1::part1(&instruct, &mut stack_array_p1);
    let end_p1 = SystemTime::now();
    // Second part
    let start_p2 = SystemTime::now();
    part2::part2(&instruct, &mut stacks_array);
    let end_p2 = SystemTime::now();
    let diff_p1 = end_p1.duration_since(start_p1).unwrap().as_micros();
    let diff_p2 = end_p2.duration_since(start_p2).unwrap().as_micros();

    println!("P1 Time: {}µs", diff_p1);
    println!("P2 Time: {}µs", diff_p2);
}
