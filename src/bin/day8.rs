// https://adventofcode.com/2021/day/8

#[macro_use]
extern crate lazy_static;

use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

lazy_static! {
  static ref UNIQ_LENS: HashSet<i32> = HashSet::from([2, 4, 3, 7]);
}

fn main() {
    let filename: String = env::args().into_iter().nth(1).unwrap();
    let file = File::open(filename).expect("Can't open file");
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let outputs: Vec<&str> = lines
        .iter()
        .map(|line| line.split("|").nth(1).unwrap().trim()).collect();
    let mut num_1478 = 0;
    for output in outputs {
        let uniq_outputs: Vec<&str> = output
            .split(" ")
            .filter(|&s| UNIQ_LENS.contains(&(s.len() as i32)))
            .collect();
        num_1478 += uniq_outputs.len();
    }
    println!("Num unique {}", num_1478);
}
