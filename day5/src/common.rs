use regex::{Captures, Regex};
use std::fs;
pub fn open() -> Vec<String> {
    let s = match fs::read_to_string("input") {
        Ok(s) => s,
        Err(_) => {
            panic!("Error while reading file!")
        }
    };
    // Split the input into two parts: the first one is the stack,
    // the second one are the instructions
    let v = s
        .split("\n\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    v
}
// Parse the string stack to an array of vectors
pub fn stacks(s: &String) -> [Vec<char>; 9] {
    // Parse with regex, each expresion inside a parethesis () represents a group.
    let re = Regex::new(
        r"([ \[\]A-Z]{3}) ([ \[\]A-Z]{3}) ([ \[\]A-Z]{3}) ([ \[\]A-Z]{3}) ([ \[\]A-Z]{3}) ([ \[\]A-Z]{3}) ([ \[\]A-Z]{3}) ([ \[\]A-Z]{3}) ([ \[\]A-Z]{3})",
    )
    .unwrap();
    // Create my array (size 9 because of 9 columns on the stack) of vector
    let mut v: [Vec<char>; 9] = Default::default();

    // For each line in the stack string, parse it using "re" variable
    // then with that result, call the function add_elements which takes
    // a vector and a Captures type
    for (i, l) in s.lines().rev().enumerate() {
        if i != 0 {
            let parsed = re.captures(l).unwrap();
            add_elements(parsed, &mut v);
        }
    }
    v
}

fn add_elements(cap: Captures, v: &mut [Vec<char>; 9]) {
    // For each element in the cap variable, if it's not the first element,
    // item will be a string either containing "[<char>]" or "   " (3 blank spaces)
    // if the item is not a blank string, and the char inside it to the v (vector)
    // variable. j represents the column in which the char is.
    for (j, item) in cap.iter().enumerate() {
        if j != 0 {
            let s = item.unwrap().as_str().to_string().chars().nth(1).unwrap();
            if s != ' ' {
                v[j - 1].push(s);
            }
        }
    }
}

pub fn instructions(s: &String) -> Vec<(i32, i32, i32)> {
    // Regex to parse instructions
    let re = Regex::new(r"move ([0-9]*) from ([0-9]*) to ([0-9]*)").unwrap();
    // For each line in s (raw string of instructions from "input" file), process that
    // line using the "re" regex and create a tuple of 3 i32 numbers
    let v = s
        .lines()
        .map(|s| {
            let cap = re.captures(s);
            let item = cap.unwrap();
            let t = (
                (&item[1]).parse::<i32>().unwrap(),
                (&item[2]).parse::<i32>().unwrap(),
                (&item[3]).parse::<i32>().unwrap(),
            );
            t
        })
        // Collect all the tuples from each line to a vector of tuples
        .collect::<Vec<_>>();
    v
}
