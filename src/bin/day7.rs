use adventofcode2021::median;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let filename: String = env::args().into_iter().nth(1).unwrap();

    let file = File::open(filename).expect("Can't open file");
    let reader = io::BufReader::new(file);
    let mut positions: Vec<i64> = reader
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    let m = median(&mut positions);
    println!("median {}, total gas: {}", m, positions.iter().map(|pos| (pos - m).abs()).sum::<i64>());
} 
