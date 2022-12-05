use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("./input.txt").expect("Couldn't open file");
    let reader = BufReader::new(file);

    let mut pairs = Pairs { ranges: Vec::new() };

    for line in reader.lines() {
        line.unwrap()
            .split(',')
            .for_each(|a| {
            a.split('-')
                .for_each(|a|
                    pairs.ranges.push(a.parse::<u8>().expect("Couldn't parse")));
        });
    }

    let mut result = pairs.count_containing_pairs();
    println!("Result: {}", result);

    result = pairs.count_overlapping_pairs();
    println!("Result: {}", result);
}

struct Pairs {
    ranges: Vec<u8>,
}

impl Pairs {
    fn count_containing_pairs(&self) -> i32 {
        let mut total: i32 = 0;

        let mut i: usize = 0;
        loop {
            if i == self.ranges.len() {
                break;
            }

            if (self.ranges[i] <= self.ranges[i + 2]
                && self.ranges[i + 1] >= self.ranges[i + 3])
                || (self.ranges[i] >= self.ranges[i + 2]
                && self.ranges[i + 1] <= self.ranges[i + 3]) {
                total += 1;
            }

            i += 4;
        }

        return total;
    }

    fn count_overlapping_pairs(&self) -> i32 {
        let mut total: i32 = 0;

        let mut i: usize = 0;
        loop {
            if i == self.ranges.len() {
                break;
            }

            if (self.ranges[i] >= self.ranges[i + 2] && self.ranges[i] <= self.ranges[i + 3])
                || (self.ranges[i + 2] <= self.ranges[i + 1] && self.ranges[i + 2] >= self.ranges[i]) {
                total += 1;
            }

            i += 4;
        }

        return total;
    }
}