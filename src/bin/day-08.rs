use std::cmp::max;
use aoc2022::grid::Grid;
use aoc2022::common::{read_input_lines};
use aoc2022::coord::Pt;

fn visible_distance_score(grid: &Grid<u8>, x: usize, y: usize) -> u64 {
    let mut score = 1;
    for (vx, vy) in [(0_i8, 1), (1, 0), (0, -1), (-1, 0)] {
        let t = visible_distance(grid, grid[Pt(x, y)], x, y, vx, vy);
        score *= t;
    }
    score
}

fn visible_distance(grid: &Grid<u8>, start: u8, x: usize, y: usize, vx: i8, vy: i8) -> u64 {
    let xx = (x as i64 + vx as i64) as usize;
    let yy = (y as i64 + vy as i64) as usize;
    if let Ok(&next) = grid.get(Pt(xx, yy)) {
        if start <= next {
            1
        } else {
            let result = visible_distance(grid, start, xx, yy, vx, vy);
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
        let mut max = grid[Pt(0, y)];
        for x in 1..grid.width-1 {
            let t = grid[Pt(x, y)];
            if t > max {
                mark_grid[Pt(x, y)] = 1;
                max = t;
            }
        }

        let mut max = grid[Pt(grid.width-1, y)];
        for x in (1..grid.width-1).rev() {
            let t = grid[Pt(x, y)];
            if t > max {
                mark_grid[Pt(x, y)] = 1;
                max = t;
            }
        }
    }

    for x in 1..grid.width-1 {
        let mut max = grid[Pt(x, 0)];
        for y in 1..height-1 {
            let t = grid[Pt(x, y)];
            if t > max {
                mark_grid[Pt(x, y)] = 1;
                max = t;
            }
        }

        let mut max = grid[Pt(x, grid.height-1)];
        for y in (1..grid.height-1).rev() {
            let t = grid[Pt(x, y)];
            if t > max {
                mark_grid[Pt(x, y)] = 1;
                max = t;
            }
        }
    }

    let total: u64 = mark_grid.iter().map(|x| *x as u64).sum::<u64>() + (grid.width * 2 + grid.height * 2 - 4) as u64;
    println!("{}", total);

    let mut _max = 0;
    for x in 1..grid.width-1 {
        for y in 1..grid.height-1 {
            _max = max(_max, visible_distance_score(&grid, x, y));
        }
    }
    println!("{}", _max);
}