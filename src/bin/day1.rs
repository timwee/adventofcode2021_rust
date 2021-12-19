use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_file = &args[1];
    println!("Input File: {}", input_file);

    if let Ok(lines) = adventofcode2021::read_lines(input_file) {
        // Consumes the iterator, returns an (Optional) String
        let summary = lines.fold((0, i32::MIN), 
        |(acc, prev), line| {
            if let Ok(num) = line {
                let num = num.parse::<i32>().unwrap();
                if prev != i32::MIN && prev < num {
                    return (acc+1, num);
                } else {
                    return (acc, num);
                }
            }
            return (0, acc);
        });
        println!("Number of increases = {}", summary.0);
        // for (idx, line) in lines.enumerate() {
        //     if let Ok(num) = line {
        //         println!("line {}, {}", idx, num);
        //         let num = num.parse::<i32>().unwrap();
        //         if prev != i32::MIN && prev < num {
        //             num_increased += 1;
        //         }
        //         prev = num;
        //     }
        // }
    }
    
}

