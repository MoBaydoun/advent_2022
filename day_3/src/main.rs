use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;
use substring::Substring;

fn main() {
    let file = File::open("./input.txt").expect("Couldn't open file");
    let reader = BufReader::new(file);

    let mut rutsacks = Rutsacks {
        sacks: Vec::new(),
        first_compartments: Vec::new(),
        second_compartments: Vec::new(),
        shared_gifts: Vec::new(),
        badges: Vec::new(),
    };

    for line in reader.lines() {
        let temp = line.unwrap();
        let first = temp.substring(0, temp.len() / 2).to_string();
        let second = temp.substring(temp.len() / 2, temp.len()).to_string();
        rutsacks.sacks.push(temp);
        rutsacks.first_compartments.push(first);
        rutsacks.second_compartments.push(second);
    }

    let mut result = rutsacks.get_shared().count_shared();
    println!("Result: {}", result);

    result = rutsacks.get_badges().count_badges();
    println!("Result: {}", result);
}

struct Rutsacks {
    sacks: Vec<String>,
    first_compartments: Vec<String>,
    second_compartments: Vec<String>,
    shared_gifts: Vec<char>,
    badges: Vec<char>,
}

impl Rutsacks {
    fn get_shared(&mut self) -> &Self {
        zip(&self.first_compartments, &self.second_compartments).for_each(|(a, b)| {
            let first = a.chars();
            let second = b.chars();
            'find_shared: for i in first.clone() {
                for j in second.clone() {
                    if i.eq(&j) {
                        self.shared_gifts.push(i);
                        break 'find_shared;
                    }
                }
            }
        });
        return self;
    }

    fn count_shared(&self) -> i32 {
        let mut total = 0;
        self.shared_gifts
            .iter()
            .for_each(|a| total += char_to_num(&a));
        return total;
    }

    fn get_badges(&mut self) -> &Self {
        let mut i = 0;
        loop {
            if i == self.sacks.len() {
                break;
            }
            let first = self.sacks[i].chars();
            let second = self.sacks[i + 1].chars();
            let third = self.sacks[i + 2].chars();
            'find_badge: for j in first.clone() {
                for k in second.clone() {
                    for l in third.clone() {
                        if j.eq(&k) && j.eq(&l) {
                            self.badges.push(j);
                            break 'find_badge;
                        }
                    }
                }
            }
            i += 3;
        }
        return self;
    }

    fn count_badges(&self) -> i32 {
        let mut total = 0;
        self.badges.iter().for_each(|a| total += char_to_num(&a));
        return total;
    }
}

fn char_to_num(c: &char) -> i32 {
    if c.is_uppercase() {
        return *c as i32 - 38;
    } else {
        return *c as i32 - 96;
    }
}
