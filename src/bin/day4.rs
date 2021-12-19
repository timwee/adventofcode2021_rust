// https://adventofcode.com/2021/day/4
// https://docs.rs/ndarray/0.11.2/ndarray/doc/ndarray_for_numpy_users/index.html

use ndarray::{prelude::*, OwnedRepr};
use ndarray::{Array, Ix3};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

const H: usize = 5;
const W: usize = 5;

#[derive(Default, Debug)]
struct Coord(usize, usize, usize);

fn parse_draw_numbers(line: &str) -> Vec<i32> {
    line.split(',').map(|s| s.parse::<i32>().unwrap()).collect()
}

fn parse_boards(
    lines: &[String],
    board_vals: &mut ArrayBase<OwnedRepr<i32>, Ix3>,
) -> HashMap<i32, Vec<Coord>> {
    let mut incidence_mat = HashMap::new();
    let mut board_num: usize = 0;
    let mut row_num: usize = 0;
    for line in lines {
        println!("line: {}", line);
        if line.is_empty() {
            board_num += 1;
            row_num = 0;

            continue;
        }
        for (col_num, num) in line
            .split(' ')
            .filter(|&x| !x.is_empty())
            .map(|s| s.parse::<i32>().unwrap())
            .enumerate()
        {
            incidence_mat
                .entry(num)
                .or_insert(Vec::new())
                .push(Coord(board_num, row_num, col_num));
            board_vals[[board_num, row_num, col_num]] = num;
        }
        row_num += 1;
    }
    return incidence_mat;
}

// fn check_winner<S, D>(coord: &Coord, boards: &ArrayBase<S, D>) -> bool
// where
//   S: DataOwned,
//   D: Dimension
//  {
//   let cur_board = boards.slice(s![coord.0, .., ..]);
//   return cur_board.row(coord.1).iter().sum() == W || cur_board.column(coord.2).iter().sum() == H
// }

fn main() {
    // Read input
    let filename: String = env::args().into_iter().nth(1).unwrap();

    let file = File::open(filename).expect("Can't open file");
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let draw_numbers = parse_draw_numbers(&lines[0]);
    println!("draw numbers: {:?}", draw_numbers);

    let num_boards = (lines.len() - 1) / 6;
    let mut boards = Array::<i32, Ix3>::ones((num_boards, H, W).f());
    let mut board_vals = Array::<i32, Ix3>::zeros((num_boards, H, W).f());
    let incidence_mat = parse_boards(&lines[2..], &mut board_vals);

    let mut board_completed = vec![false; num_boards];
    let mut has_any_board_completed = false;

    for (num_draw, num) in draw_numbers.iter().enumerate() {
        match incidence_mat.get(&num) {
            Some(coords) => {
                for coord in coords {
                    if board_completed[coord.0] {
                        continue;
                    }
                    boards[[coord.0, coord.1, coord.2]] = 0;
                    let cur_board = boards.slice(s![coord.0, .., ..]);
                    if cur_board.row(coord.1).iter().sum::<i32>() == (0 as i32)
                        || cur_board.column(coord.2).iter().sum::<i32>() == (0 as i32)
                    {
                        board_completed[coord.0] = true;
                        let winning_val =
                            compute_winning_val(&mut boards, &mut board_vals, &coord, *num as i64);
                        if has_any_board_completed {
                            println!(
                                "completed on {}th draw with value {}: {:?}, {:?}",
                                num_draw, num, coord, winning_val
                            );
                        } else {
                            println!(
                                "**** winner found on {}th draw with value {}: {:?}, {:?}",
                                num_draw, num, coord, winning_val
                            );
                            has_any_board_completed = true;
                        }
                    }
                }
            }
            None => println!("No coordinates found for {:?}", num),
        }
    }
    // println!("no winner found!");
    // std::process::exit(1);
}

fn compute_winning_val(
    boards: &mut ArrayBase<OwnedRepr<i32>, Dim<[usize; 3]>>,
    board_vals: &mut ArrayBase<OwnedRepr<i32>, Dim<[usize; 3]>>,
    coord: &Coord,
    num: i64,
) -> i64 {
    // let masked_board_vals = boards.slice(s![coord.0, .., ..]) * boards.slice(s![coord.0, .., ..]);
    // println!("{:?}", masked_board_vals);
    let mask = boards.index_axis_mut(Axis(0), coord.0);
    let board_val = board_vals.index_axis_mut(Axis(0), coord.0);
    println!("mask {:?}", mask);
    println!("board_val {:?}", board_val);

    let masked = &mask * &board_val;
    let sum = masked.sum();
    println!("masked {:?}, sum {:?}, num {}", masked, sum, num);
    return (sum as i64) * num;
}
