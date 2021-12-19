use circular_queue::CircularQueue;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_file = &args[1];
    println!("Input File: {}", input_file);

    let mut result = 0;

    if let Ok(lines) = adventofcode2021::read_lines(input_file) {
        let mut buffer = CircularQueue::with_capacity(3);
        
        let mut tot3 = 0;

        for line in lines {
            if let Ok(num) = line {
                let num = num.parse::<i32>().unwrap();
                if !buffer.is_full() {
                    buffer.push(num);
                    tot3 += num;
                } else {
                    let prev_tot3 = tot3;
                    let popped = buffer.push(num).unwrap();
                    tot3 = (tot3 - popped) + num;
                    if prev_tot3 < tot3 {
                        result += 1;
                    }
                }
            }
        }
        println!("result is {}", result);
    }
}