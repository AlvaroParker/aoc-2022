use std::fs;

fn main() {
    let input = open();
    let v2d = parse_input(input);
    part1(&v2d);
    part2(&v2d);
}

fn part1(v2d: &Vec<Vec<u32>>) {
    // parse_input() returns a 2d <u32> vector, then I map over it and calculate the
    // sum of each vector, finally I get the max using .max()
    let v = v2d.iter().map(|x| x.iter().sum::<u32>()).max();
    println!("Greatest calories: {}", v.unwrap_or(0));
}

fn part2(v2d: &Vec<Vec<u32>>) {
    // iterate over 2d <u32> vector calculate the sum of each internal vector
    let mut v = v2d
        .iter()
        .map(|x| x.iter().sum::<u32>())
        .collect::<Vec<u32>>();
    // sort generated vector
    v.sort();
    // reverse vector, greatest values come first
    v.reverse();

    // calculate total sum of top 3 values
    let mut total = 0;
    for i in 0..3 {
        total += v.get(i).unwrap();
    }

    // output
    println!(
        "Top 3:\n1. {}\n2. {}\n3. {}\nTotal: {}",
        v.get(0).unwrap(),
        v.get(1).unwrap(),
        v.get(2).unwrap(),
        total
    );
}

fn open() -> String {
    match fs::read_to_string("input") {
        Ok(content) => {
            return content;
        }
        Err(err) => {
            panic!("Error while reading file: {}", err);
        }
    };
}

fn parse_input(s: String) -> Vec<Vec<u32>> {
    // Split input with 2 new line chars as separator
    let v = s.split("\n\n").collect::<Vec<&str>>();
    // Iterate over the generated v vector and split with \n as separator, then parse to u32 each
    // line
    v.iter()
        .map(|s| {
            s.split_whitespace()
                .map(|x| x.parse::<u32>().unwrap_or(0))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}
