use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn mean(numbers: &Vec<i64>) -> f64 {

    let sum: i64 = numbers.iter().sum();

    sum as f64 / numbers.len() as f64

}

pub fn median(numbers: &mut Vec<i64>) -> i64 {

  numbers.sort();

  let mid = numbers.len() / 2;
  if numbers.len() % 2 == 0 {
      mean(&vec![numbers[mid - 1], numbers[mid]]) as i64
  } else {
      numbers[mid]
  }

}