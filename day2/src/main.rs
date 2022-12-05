use std::fs;
// A, X -> ROCK 1
// B, Y -> PAPER 2
// C, Z -> SIZOR 3
// DRAW -> 3, WON -> 6, LOST -> 0

pub struct Game {
    player: char,
    you: char,
}
impl Game {
    pub fn who_won(&self) -> u32 {
        match self.player {
            'A' => match self.you {
                'X' => 4,
                'Y' => 8,
                'Z' => 3,
                _ => 0,
            },
            'B' => match self.you {
                'X' => 1,
                'Y' => 5,
                'Z' => 9,
                _ => 0,
            },
            'C' => match self.you {
                'X' => 7,
                'Y' => 2,
                'Z' => 6,
                _ => 0,
            },
            _ => 0,
        }
    }
    pub fn score_p2(&self) -> u32 {
        match self.player {
            'A' => match self.you {
                'X' => 3,
                'Y' => 4,
                'Z' => 8,
                _ => 0,
            },
            'B' => match self.you {
                'X' => 1,
                'Y' => 5,
                'Z' => 9,
                _ => 0,
            },
            'C' => match self.you {
                'X' => 2,
                'Y' => 6,
                'Z' => 7,
                _ => 0,
            },
            _ => 0,
        }
    }

    pub fn from_str(s: &str) -> Game {
        let v = s.split_whitespace().collect::<Vec<_>>();
        Game {
            player: v.get(0).unwrap().chars().nth(0).unwrap(),
            you: v.get(1).unwrap().chars().nth(0).unwrap(),
        }
    }
}

fn main() {
    let v = open();
    part1(&v);
    part2(&v);
}

fn open() -> Vec<String> {
    let content = match fs::read_to_string("input") {
        Ok(s) => s,
        Err(err) => {
            panic!("Error while reading file: {}", err)
        }
    };
    content.split("\n").map(str::to_string).collect::<Vec<_>>()
}

fn part1(input: &Vec<String>) {
    let cmp = String::from("");
    let p = input
        .iter()
        .map(|s| {
            if s != &cmp {
                Game::from_str(s).who_won()
            } else {
                0
            }
        })
        .collect::<Vec<u32>>()
        .iter()
        .sum::<u32>();
    println!("P1 Final points: {}", p);
}

fn part2(input: &Vec<String>) {
    let cmp = String::from("");
    let p = input
        .iter()
        .map(|s| {
            if s != &cmp {
                Game::from_str(s).score_p2()
            } else {
                0
            }
        })
        .collect::<Vec<u32>>()
        .iter()
        .sum::<u32>();
    println!("P2 Final points: {}", p);
}
