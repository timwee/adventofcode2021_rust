use std::env;

struct Position(i32, i32);

fn main() {
    let input_file: String = env::args().into_iter().nth(1).unwrap();

    let mut start_pos: Position = Position(0, 0);
    if let Ok(lines) = adventofcode2021::read_lines(input_file) {
        for line in lines {
            if let Ok(line) = line {
                let toks: Vec<&str> = line.split(" ").collect();
                if toks.len() != 2 {
                    println!("expected only 2 tokens in line, but got '{}'", line);
                    std::process::exit(1);
                }
                process_cmd(&mut start_pos, toks[0], toks[1].parse::<i32>().unwrap());
            }
        }
    }
    println!("ending position: ({}, {}), {}", start_pos.0, start_pos.1, start_pos.0 * start_pos.1);
}

fn process_cmd(pos: &mut Position, cmd: &str, move_amt: i32) {
    match cmd {
        "forward" => pos.0 += move_amt,
        "up" => pos.1 -= move_amt,
        "down" => pos.1 += move_amt,
        _ => println!("unexpected cmd encountered: {}", cmd),
    };
}