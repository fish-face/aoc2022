use std::cmp::max;

use aoc2022::common::{parse_input_lines, read_input_lines};
use aoc2022::coord::Pt;
use aoc2022::grid::Grid;

static mut MAX_X: usize = 0;
static mut MAX_Y: usize = 0;

peg::parser! {
    grammar input_parser() for str {
        pub rule line() -> Vec<Pt<usize>>
            = (segment() ** " -> ")

        rule segment() -> Pt<usize>
            = x:num() "," y:num() {
                unsafe {
                    MAX_X = max(MAX_X, x);
                    MAX_Y = max(MAX_Y, y);
                    // println!("{} {}", MAX_X, MAX_Y);
                }
                Pt(x, y)
        }

        rule num() -> usize
            = n:$(['0'..='9']+) {? n.parse().or(Err("bad number"))}
    }
}

fn pour(grid: &mut Grid<char>, floor: bool) -> bool {
    let mut pos = Pt(500, 0);
    if grid[pos] != '.' {
        return false;
    }
    let mut result = false;
    'outer: loop {
        for offset in vec![Pt(0_isize, 1), Pt(-1, 1), Pt(1, 1)] {
            // why even bother
            let test = Pt(
                (pos.0 as isize + offset.0) as usize,
                (pos.1 as isize + offset.1) as usize,
            );
            match grid.get(test) {
                Ok(&'.') => {
                    pos = test;
                    continue 'outer;
                }
                Ok(_) => {
                    continue;
                }
                Err(_) => {
                    if floor {
                        continue
                    } else {
                        break 'outer;
                    }
                }
            }
        }
        result = true;
        break;
    }
    if result {
        grid[pos] = '~';
    }
    result
}

fn main() {
    let input = read_input_lines()
        .expect("Could not read input")
        .map(|l| input_parser::line(&l).unwrap())
        .collect::<Vec<_>>();

    let max_x = input.iter().flatten().map(|p| p.0).max().unwrap();
    let max_y = input.iter().flatten().map(|p| p.1).max().unwrap();
    // println!("{} {}", max_x, max_y);
    // println!("{:?}", input);

    let mut grid1 =
        Grid::<char>::from_data(max_x + 1, max_y + 1, vec!['.'; (max_x + 1) * (max_y + 1)]);
    let mut grid2 =
        Grid::<char>::from_data(max_x * 2, max_y + 2, vec!['.'; (max_x * 2) * (max_y + 2)]);
    for line in input {
        let mut prev = line[0];
        for cur in &line[1..] {
            if prev.0 < cur.0 {
                for p in (prev.0..=cur.0).map(|i| Pt(i, prev.1)) {
                    grid1[p] = '#';
                    grid2[p] = '#';
                }
            } else if prev.0 > cur.0 {
                for p in (cur.0..=prev.0).rev().map(|i| Pt(i, prev.1)) {
                    grid1[p] = '#';
                    grid2[p] = '#';
                }
            } else if prev.1 < cur.1 {
                for p in (prev.1..=cur.1).map(|i| Pt(prev.0, i)) {
                    grid1[p] = '#';
                    grid2[p] = '#';
                }
            } else {
                for p in (cur.1..=prev.1).rev().map(|i| Pt(prev.0, i)) {
                    grid1[p] = '#';
                    grid2[p] = '#';
                }
            }
            prev = *cur;
        }
    }

    let mut i = 0;
    while pour(&mut grid1, false) {
        i += 1;
    }
    println!("{}", i);

    let mut i = 0;
    while pour(&mut grid2, true) {
        i += 1;
    }
    println!("{}", i);
}
