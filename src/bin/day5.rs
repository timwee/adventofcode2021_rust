// https://adventofcode.com/2021/day/5
// https://siciarz.net/24-days-rust-static-initialization/
#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

lazy_static! {
    static ref RE: Regex = Regex::new(r"([\d]+),([\d]+) \-> ([\d]+),([\d]+)").unwrap();
}

fn parse_line(s: &str) -> Option<(i32, i32, i32, i32)>  {
  for caps in RE.captures(s) {
    return Some((
      caps.get(1).unwrap().as_str().parse::<i32>().unwrap(),
      caps.get(2).unwrap().as_str().parse::<i32>().unwrap(),
      caps.get(3).unwrap().as_str().parse::<i32>().unwrap(),
      caps.get(4).unwrap().as_str().parse::<i32>().unwrap()));
  }
  return None;
}

fn mark_seen(mat: &mut HashMap::<(i32, i32), i32>, x1: i32, y1: i32, x2: i32, y2: i32) {
  // println!("{} {} -> {} {}", x1, y1, x2, y2);
  let steps = std::cmp::max((x2 - x1).abs(), (y2 - y1).abs());
  let dx = (x2 - x1) / steps;
  let dy = (y2 - y1) / steps;
  for i in 0..steps+1 {
    let coord = (x1 + dx * i, y1 + dy * i);
    let c: i32 = *mat.get(&coord).unwrap_or(&0);
    mat.insert(coord, c + 1);
  }      
}

fn count_intersections(mat: &HashMap::<(i32, i32), i32>) -> usize {
  mat.into_iter().map(|(&_k, &v)| v).filter(|&v| v > 1).count()
}

fn main() {
  let filename: String = env::args().into_iter().nth(1).unwrap();

  let file = File::open(filename).expect("Can't open file");
  let reader = io::BufReader::new(file);
  let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

  let mut mat1 = HashMap::<(i32, i32), i32>::new();
  let mut mat2 = HashMap::<(i32, i32), i32>::new();

  for line in lines {
    if let Some((x1, y1, x2, y2)) = parse_line(&line) {
      if x1 == x2 || y1 == y2 {
        mark_seen(&mut mat1, x1, y1, x2, y2);
      }
      mark_seen(&mut mat2, x1, y1, x2, y2);
    }
  }
  println!("{} {}", count_intersections(&mat1), count_intersections(&mat2)); 
}
