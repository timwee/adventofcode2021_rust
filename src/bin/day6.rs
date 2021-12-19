// https://adventofcode.com/2021/day/6
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

const DAYS: i64 = 256;

fn parse_initial_ages(s: &str) -> HashMap<i64, i64> {
    let mut fish_per_day = HashMap::<i64, i64>::new();
    for age in s.split(',').map(|s| s.parse::<i64>().unwrap()) {
        let c = *fish_per_day.get(&age).unwrap_or(&0);
        fish_per_day.insert(age, c + 1);
    }
    return fish_per_day;
}

fn main() {
    let filename: String = env::args().into_iter().nth(1).unwrap();

    let file = File::open(filename).expect("Can't open file");
    let reader = io::BufReader::new(file);
    let line = reader.lines().next().unwrap().unwrap();

    let mut fish_per_day = parse_initial_ages(&line);
    for cur_day in 0..DAYS {
        println!(
            "DAY {} start: {:?}, sum: {}",
            cur_day,
            fish_per_day,
            fish_per_day.iter().map(|(_, &v)| v).sum::<i64>()
        );
        let new_fish = *fish_per_day.get(&0).unwrap_or(&0);
        let mut buffer = HashMap::<i64, i64>::new();
        buffer.insert(8, new_fish);
        for (k, v) in fish_per_day {
            let new_key = if (k - 1) < 0 { 6 } else { k - 1 };
            let old_val = *buffer.entry(new_key).or_default();
            buffer.insert(new_key, old_val + v);
        }
        fish_per_day = buffer;
        println!(
            "DAY {} end: {:?}, sum: {}\n",
            cur_day,
            fish_per_day,
            fish_per_day.iter().map(|(_, &v)| v).sum::<i64>()
        );
    }
    println!("End of {} days: {:?}", DAYS, fish_per_day);
    println!(
        "num_fish at end of {} days: {}",
        DAYS,
        fish_per_day.iter().map(|(_, &v)| v).sum::<i64>()
    );
}
