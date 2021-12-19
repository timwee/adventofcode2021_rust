use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::str::Chars;

fn process(items: Vec<String>, flip: bool, pos: usize) -> String {
    if items.len() == 1 {
      println!("returning");
      items[0].clone()
    } else {
      let ct1 = items.iter().filter(|item| item.as_bytes()[pos] == b'1').count();
      let ge1 = 2 * ct1 >= items.len();
      let bit = if flip ^ ge1 { b'0'} else { b'1' };
      let items_filtered : Vec<String> = items.into_iter().filter(|item| item.as_bytes()[pos] == bit).collect::<Vec<_>>();
      assert!(items_filtered.len() >= 1);
      process(items_filtered, flip, pos+1)
    }
  }
  
  fn bin2int(s: String) -> i32 {
    let mut num: i32 = 0;
    let c = s.as_bytes();
    for i in 0..c.len() {
      num |= ((c[c.len() - i - 1] == b'1') as i32) << i;
    }
    num
  }
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).expect("Can't open file");
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    let mut count = vec![0; lines[0].len()];
    for line in &lines {
        for i in line
            .chars()
            .enumerate()
            .filter(|&(_i, c)| c == '1')
            .map(|(i, _)| i)
        {
            count[i] += 1;
        }
    }
    count.reverse();
    let mut gamma: i32 = 0;
    let mut epsilon: i32 = 0;
    for (i, c) in count.iter().enumerate() {
        if 2 * c > lines.len() {
            gamma |= 1 << i;
        } else if 2 * c < lines.len() {
            epsilon |= 1 << i;
        } else if 2 * c == lines.len() {
            println!("Tie in position {}", i);
        }
    }
    println!(
        "gamma = {} epsilon = {} gamma * epsilon = {}",
        gamma,
        epsilon,
        gamma * epsilon
    );

    // part 2
    let elem_idxs = lines
        .iter()
        .enumerate()
        .map(|(i, _e)| i as i32)
        .collect::<Vec<_>>();
    println!("{:?}", elem_idxs);

    // let line_chars: Vec<Chars> = lines.iter().map(|l| l.chars()).collect();

    // let oxygen = lines[process(&line_chars, elem_idxs, '1', 0, )[0]];
    let oxygen_gen = process(lines.clone(), true, 0);
    let co2_gen = process(lines.clone(), false, 0);
    let oxygen_gen_int = bin2int(oxygen_gen);
    let co2_gen_int = bin2int(co2_gen);
    println!("oxygen = {}, co2 = {}, product = {}", oxygen_gen_int, co2_gen_int, oxygen_gen_int * co2_gen_int);
}

// fn process(line_chars: &Vec<Chars>, elem_idxs: Vec<i32>, needle: char, pos: i32) -> {
//     if 
// }