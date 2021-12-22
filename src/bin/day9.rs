use std::env;
use std::fs::File;
use std::io::{self, BufRead};
#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref DELTA_XY: Vec<(i32, i32)> = vec![(-1, 0), (1, 0), (0, 1), (0, -1)];
}

#[derive(Default, Debug)]
struct Coord(usize, usize);

fn neighbors(cur_x: usize, cur_y: usize, rows: i32, cols: i32) -> Vec<(usize, usize)> {
    let coords = DELTA_XY.iter().map(|(dx, dy)| {
        return ((cur_x as i32) + dx, (cur_y as i32) + dy);
    });
    return coords
        .filter(|(x, y)| *x >= 0 && *x < rows && *y >= 0 && *y < cols)
        .map(|(x, y)| (x as usize, y as usize))
        .collect();
}

fn main() {
    let filename: String = env::args().into_iter().nth(1).unwrap();

    let file = File::open(filename).expect("Can't open file");
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    assert!(lines.len() > 0);
    let rows = lines.len();
    let cols = lines[0].len();
    let mut grid = vec![vec![0; cols]; rows];
    let mut visited = vec![vec![false; cols]; rows];
    fill_grid(&lines, &mut grid);

    let mut basins: Vec<i32> = Vec::new();
    let mut basin_coords: Vec<Coord> = Vec::new();
    let mut basin_sizes: Vec<usize> = Vec::new();
    for cur_row in 0..rows {
        for cur_col in 0..cols {
            if neighbors(cur_row, cur_col, rows as i32, cols as i32)
                .iter()
                .all(|(n_x, n_y)| grid[*n_x][*n_y] > grid[cur_row][cur_col])
            {
                basins.push(grid[cur_row][cur_col]);
                basin_coords.push(Coord(cur_row, cur_col));

                let mut basin_size = 0;
                let mut q = vec![(cur_row, cur_col)];
                while !q.is_empty() {
                    let (qx, qy) = q.pop().unwrap();
                    basin_size += 1;
                    for (qx2, qy2) in neighbors(qx, qy, rows as i32, cols as i32) {
                        if grid[qx as usize][qy as usize] < grid[qx2 as usize][qy2 as usize]
                            && grid[qx2 as usize][qy2 as usize] < 9
                            && !visited[qx2 as usize][qy2 as usize]
                        {
                            visited[qx2 as usize][qy2 as usize] = true;
                            q.push((qx2, qy2));
                        }
                    }
                }
                basin_sizes.push(basin_size);
            }
        }
    }

    // let largest_sizes: i32 = largest_basin_sizes(&basin_coords, &mut basins, 3);
    println!(
        "risk level: {:?}",
        basins.iter().map(|b| b + 1).sum::<i32>()
    );

    basin_sizes.sort();
    basin_sizes.reverse();
    println!(
        "{:?} {}",
        basin_sizes,
        basin_sizes[0] * basin_sizes[1] * basin_sizes[2]
    );
}

fn fill_grid(lines: &Vec<String>, grid: &mut Vec<Vec<i32>>) {
    for (row, line) in lines.iter().enumerate() {
        for (col, chr) in line.chars().enumerate() {
            grid[row][col] = chr.to_digit(10).unwrap() as i32;
        }
    }
}
