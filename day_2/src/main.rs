use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;

fn main() {
    let file = File::open("./input.txt")
        .expect("Couldn't open file");
    let reader = BufReader::new(file);

    let mut rounds = Rounds {
        elf_hands: Vec::new(),
        hands: Vec::new()
    };

    for line in reader.lines() {
        let split: Vec<char> = line.unwrap()
            .split(' ')
            .map(|a| a.parse::<char>().expect("Couldn't parse"))
            .collect();
        rounds.elf_hands.push(split[0]);
        rounds.hands.push(split[1]);
    }

    let mut result = rounds.strategy_guide();
    println!("Result: {}", result);

    result = rounds.weird_strategy_guide();
    println!("Result: {}", result);

}

struct Rounds {
    elf_hands: Vec<char>,
    hands: Vec<char>
}

impl Rounds {
    fn print(&self) {
        zip(&self.elf_hands, &self.hands)
            .for_each(|(a, b)| println!("{} : {}", a, b));
    }

    fn strategy_guide(&self) -> i32 {
        let mut score = 0;
        zip(&self.elf_hands, &self.hands)
            .for_each(|(a, b)| { match a {
                'A' => match b {
                    'X' => {score += 4},
                    'Y' => {score += 8},
                    'Z' => {score += 3},
                    _ => panic!("Oh noes")
                },
                'B' => match b {
                    'X' => {score += 1},
                    'Y' => {score += 5},
                    'Z' => {score += 9},
                    _ => panic!("Oh noes")
                },
                'C' => match b {
                    'X' => {score += 7},
                    'Y' => {score += 2},
                    'Z' => {score += 6},
                    _ => panic!("Oh noes")
                },
                _ => panic!("Something really bad happened")
            } });

        return score;
    }

    fn weird_strategy_guide(&self) -> i32 {
        let mut score = 0;
        zip(&self.elf_hands, &self.hands)
            .for_each(|(a, b)| { match a {
                'A' => match b {
                    'X' => {score += 3},
                    'Y' => {score += 4},
                    'Z' => {score += 8},
                    _ => panic!("Oh noes")
                },
                'B' => match b {
                    'X' => {score += 1},
                    'Y' => {score += 5},
                    'Z' => {score += 9},
                    _ => panic!("Oh noes")
                },
                'C' => match b {
                    'X' => {score += 2},
                    'Y' => {score += 6},
                    'Z' => {score += 7},
                    _ => panic!("Oh noes")
                },
                _ => panic!("Something really bad happened")
            } });

        return score;
    }
}
