use std::env;

#[derive(Default, Debug)]
struct Gauge {
    horizontal_pos: i32,
    depth: i32,
    aim: i32,
 }

fn main() {
    let input_file: String = env::args().into_iter().nth(1).unwrap();

    let mut gauge: Gauge = Default::default();
    if let Ok(lines) = adventofcode2021::read_lines(input_file) {
        for line in lines {
            if let Ok(line) = line {
                let toks: Vec<&str> = line.split(" ").collect();
                if toks.len() != 2 {
                    println!("expected only 2 tokens in line, but got '{}'", line);
                    std::process::exit(1);
                }
                process_cmd(&mut gauge, toks[0], toks[1].parse::<i32>().unwrap());
            }
        }
    }
    println!("ending gauge: {:?}, multiplied: {}", gauge, gauge.horizontal_pos * gauge.depth);
}

fn process_cmd(gauge: &mut Gauge, cmd: &str, move_amt: i32) {
    match cmd {
        "forward" => {
            gauge.horizontal_pos += move_amt;
            gauge.depth += gauge.aim * move_amt;
        },
        "up" => gauge.aim -= move_amt,
        "down" => gauge.aim += move_amt,
        _ => println!("unexpected cmd encountered: {}", cmd),
    };
}