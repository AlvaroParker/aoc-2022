use std::fs;

fn main() {
    let content = open();
    let v = parse(content);
    let visible = part1(&v);
    let max = part2(&v);
    println!("Trees visibles from outside the grid: {}", visible);
    println!("Highest scenic score: {}", max);
}

fn open() -> String {
    let content = match fs::read_to_string("input") {
        Ok(s) => s,
        Err(e) => {
            panic!("Error while reading file: {}", e)
        }
    };
    content
}

fn parse(s: String) -> Vec<Vec<i32>> {
    let v = s
        .split_whitespace()
        .map(|ss| {
            ss.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    v
}

fn part1(v: &Vec<Vec<i32>>) -> usize {
    let mut tmp = Vec::new();
    for (i, vec) in v.iter().enumerate() {
        for (j, n) in vec.iter().enumerate() {
            if check(&v, j, i, *n) {
                tmp.push(*n);
            }
        }
    }
    tmp.len()
}

fn part2(v: &Vec<Vec<i32>>) -> i32 {
    let mut tmp = Vec::new();
    for (i, vec) in v.iter().enumerate() {
        for (j, n) in vec.iter().enumerate() {
            let score = score(&v, j, i, *n);
            tmp.push(score);
        }
    }
    tmp.iter().max().unwrap().clone()
}

fn check(v: &Vec<Vec<i32>>, x: usize, y: usize, n: i32) -> bool {
    let row = v.get(y).unwrap();
    let (l_row, r_row) = row.split_at(x);

    let mut col = Vec::new();
    for i in v {
        col.push(i.get(x).unwrap());
    }
    let (l_col, r_col) = col.split_at(y);

    let max_r_col = match r_col.iter().skip(1).max() {
        Some(s) => *s.clone() as i32,
        None => -1,
    };
    let max_l_col = match l_col.iter().max() {
        Some(n) => *n.clone() as i32,
        None => -1,
    };
    let max_r_row = match r_row.iter().skip(1).max() {
        Some(n) => *n as i32,
        None => -1,
    };
    let max_l_row = match l_row.iter().max() {
        Some(n) => *n as i32,
        None => -1,
    };
    n > max_l_row || n > max_r_row || n > max_l_col || n > max_r_col
}
fn score(v: &Vec<Vec<i32>>, x: usize, y: usize, n: i32) -> i32 {
    let row = v.get(y).unwrap();
    let (l_row, r_row) = row.split_at(x);
    let mut col = Vec::new();
    for i in v {
        col.push(*i.get(x).unwrap());
    }
    let (l_col, r_col) = col.split_at(y);

    if r_col.len() == 1 || r_row.len() == 1 || l_row.len() == 0 || l_col.len() == 0 {
        return 0;
    };

    let mut l_col = l_col[1..].to_vec();
    let mut l_row = l_row[1..].to_vec();
    l_col.reverse();
    l_row.reverse();

    let up = filter(l_col.as_slice(), n).len();
    let left = filter(l_row.as_slice(), n).len();
    let down = filter(&r_col[1..], n).len();
    let right = filter(&r_row[1..], n).len();
    (up * down * left * right) as i32
}

fn filter(v: &[i32], n: i32) -> Vec<i32> {
    let mut tmp = Vec::new();
    for i in v {
        tmp.push(*i);
        if i >= &n {
            break;
        }
    }
    tmp
}
