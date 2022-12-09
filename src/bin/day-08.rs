use std::cmp::max;
use aoc2022::grid::Grid;
use aoc2022::common::{read_input_lines};

// fn mark_visible(mut it: impl Iterator<Item = u8>, &mut marks: Grid<bool>) {
//     let mut max = it.next().unwrap_or(0);
//     for t in it.enumerate() {
//         if t > max {
//             marks.at_mut()
//             max = t;
//         }
//     }
// }

fn visible_distance_score(grid: &Grid<u8>, x: usize, y: usize) -> u64 {
    let mut score = 1;
    for (vx, vy) in [(0_i8, 1), (1, 0), (0, -1), (-1, 0)] {
        // println!("  Next dir");
        // let mut results: Grid<u64> = Grid::new(grid.width, grid.height);
        let t = visible_distance(grid, *grid.at(x, y).unwrap(), x, y, vx, vy);
        // println!("  = {}", t);
        score *= t;
    }
    // println!("  {}", score);
    score
}

fn visible_distance(grid: &Grid<u8>, start: u8, x: usize, y: usize, vx: i8, vy: i8) -> u64 {
    let xx = (x as i64 + vx as i64) as usize;
    let yy = (y as i64 + vy as i64) as usize;
    let next = grid.at(xx, yy);

    if let Ok(next) = next {
        if start <= *next {
            // results.set(x, y, 0).unwrap();
            // println!("      Walking {},{} to {},{} is {} to {}", x, y, xx, yy, start, next);
            1
        } else {
            // println!("      Adding 1 at {}, {}", x, y);
            let result = visible_distance(grid, start, xx, yy, vx, vy);
            // results.set(x, y, 1+result).unwrap();
            1 + result
        }
    } else {
        0
    }
}

fn main() {
    let input = read_input_lines().expect("Could not read input").collect::<Vec<_>>();
    let flattened: Vec<_> = input.join("").bytes().map(|b| b - '0' as u8).collect();
    let width = input[0].len();
    let height = input.len();

    let grid: Grid<u8> = Grid::from_data(width, height, flattened);
    let mut mark_grid: Grid<u8> = Grid::new(width, height);

    for y in 1..grid.height-1 {
        let mut max = *grid.at(0, y).unwrap();
        for x in 1..grid.width-1 {
            let t = *grid.at(x, y).unwrap();
            if t > max {
                mark_grid.set(x, y, 1).unwrap();
                max = t;
            }
        }

        let mut max = *grid.at(grid.width-1, y).unwrap();
        for x in (1..grid.width-1).rev() {
            let t = *grid.at(x, y).unwrap();
            if t > max {
                mark_grid.set(x, y, 1).unwrap();
                max = t;
            }
        }
    }

    for x in 1..grid.width-1 {
        let mut max = *grid.at(x, 0).unwrap();
        for y in 1..height-1 {
            let t = *grid.at(x, y).unwrap();
            if t > max {
                mark_grid.set(x, y, 1).unwrap();
                max = t;
            }
        }

        let mut max = *grid.at(x, grid.height-1).unwrap();
        for y in (1..grid.height-1).rev() {
            let t = *grid.at(x, y).unwrap();
            if t > max {
                mark_grid.set(x, y, 1).unwrap();
                max = t;
            }
        }
    }

    let total: u64 = mark_grid.iter().map(|x| *x as u64).sum::<u64>() + (grid.width * 2 + grid.height * 2 - 4) as u64;
    // for (row, mark_row) in zip(grid.rows(), mark_grid.rows_mut()) {
    //     total += mark_visible(row.iter().copied());
    //     total += mark_visible(row.iter().rev().copied());
    // }
    // for col in grid.columns() {
    //     let col: Vec<_> = col.copied().collect();
    //     total += mark_visible(col.iter().copied());
    //     total += mark_visible(col.iter().rev().copied());
    // }
    println!("{}", total);
    // println!("{}\n", grid.to_string(Some("")));
    // println!("{}", mark_grid.to_string(Some("")));

    let mut _max = 0;
    // let x = 16;
    // let y = 25;
    for x in 1..grid.width-1 {
        for y in 1..grid.height-1 {
            // println!("{} {}:", x, y);
            _max = max(_max, visible_distance_score(&grid, x, y));
        }
    }
    println!("{}", _max);
}