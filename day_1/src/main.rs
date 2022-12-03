use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("./input.txt")
        .expect("File not found");
    let reader = BufReader::new(file);
    
    let mut numbers: Vec<i32> = Vec::new();
    
    let mut tracker = 0;
    for line in reader.lines() {
        let curr = line.unwrap();
        if curr.eq("") {
            numbers.push(tracker);
            tracker = 0;
            continue;
        }
        tracker += curr.parse::<i32>().expect("Couldn't parse");
    }

    let mut result = find_largest(&numbers);
    println!("Result: {}", result);

    result = top_three_total(&mut numbers);
    println!("Result: {}", result);
}

fn print_vec<T>(vec: Vec<T>)
where T: std::fmt::Display {
    for item in vec {
        println!("{}", item);
    }
}

fn find_largest(vec: &Vec<i32>) -> i32 {
    let mut largest: i32 = i32::MIN;
    vec.iter()
        .for_each(|i| {largest = if i > &largest {*i} else {largest}});
    return largest;
}

fn top_three_total(vec: &mut Vec<i32>) -> i32 {
    vec.sort_by(|a, b| b.cmp(a));
    return vec[0] + vec[1] + vec[2];
}