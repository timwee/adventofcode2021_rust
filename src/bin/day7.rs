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

    // part 2
    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();
    let sum_n = |n: i64| n * (n + 1) / 2; 
    let total_dist = |x: i64| positions.iter().map(|x2: &i64| sum_n((x2 - x).abs()) ).sum::<i64>();
    let total_dist_to_other: i64 = (min .. max + 1).map(total_dist).min().unwrap();
    println!("part2 min total dist {}", total_dist_to_other);
} 
