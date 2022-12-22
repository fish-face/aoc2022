use crate::Direction::*;
use aoc2022::common::read_input;
use aoc2022::coord::Pt;
use aoc2022::grid::Grid;
use lazy_static::lazy_static;
use std::cmp::max;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
enum Direction {
    Left = -1,
    Right = 1,
}

lazy_static! {
    static ref ROCK_SHAPES: [Vec<Pt<i64>>; 5] = [
        vec![Pt(0, 0), Pt(1, 0), Pt(2, 0), Pt(3, 0)],
        vec![Pt(1, 0), Pt(0, 1), Pt(1, 1), Pt(2, 1), Pt(1, 2)],
        vec![Pt(0, 0), Pt(1, 0), Pt(2, 0), Pt(2, 1), Pt(2, 2)],
        vec![Pt(0, 0), Pt(0, 1), Pt(0, 2), Pt(0, 3)],
        vec![Pt(0, 0), Pt(1, 0), Pt(0, 1), Pt(1, 1)],
    ];
}
const N_ROCKS: usize = 5;

fn offset_shape(shape: &Vec<Pt<i64>>, offset: Pt<i64>) -> Vec<Pt<i64>> {
    shape.iter().map(|p| *p + offset).collect()
}

fn test(tower: &Grid<bool>, locs: &Vec<Pt<i64>>) -> bool {
    !locs
        .iter()
        .any(|Pt(x, y)| *tower.get(Pt(*x as usize, *y as usize)).unwrap_or(&true))
}

fn set(tower: &mut Grid<bool>, locs: &Vec<Pt<i64>>) {
    locs.iter()
        .for_each(|Pt(x, y)| tower[Pt(*x as usize, *y as usize)] = true);
}

fn drop_rock<'a>(
    tower: &mut Grid<bool>,
    mut dirs: impl Iterator<Item = (usize, &'a Direction)>,
    rock: usize,
    heights: &mut [i64],
) {
    let rock_shape = &ROCK_SHAPES[rock];
    let height = *heights.iter().max().unwrap();
    let mut rock_locs = offset_shape(rock_shape, Pt(2, height + 3));
    loop {
        // push
        let (_, &dir) = dirs.next().unwrap();
        let test_rock_locs = offset_shape(&rock_locs, Pt(dir as i64, 0));
        if test(tower, &test_rock_locs) {
            rock_locs = test_rock_locs;
        }

        // fall
        let test_rock_locs = offset_shape(&rock_locs, Pt(0, -1));
        if test(tower, &test_rock_locs) {
            rock_locs = test_rock_locs;
        } else {
            set(tower, &rock_locs);
            rock_locs
                .iter()
                .for_each(|Pt(x, y)| heights[*x as usize] = max(heights[*x as usize], *y + 1));
            break;
        }
    }
}

fn part1(dirs: &Vec<Direction>, target: usize) {
    let mut tower = Grid::<bool>::new(7, target * 4 + 2);
    // let mut i = 0;
    let mut heights = [0; 7];
    let mut iter_dirs = dirs.iter().enumerate().cycle().peekable();
    for i in 0..target {
        let rock: usize = i % N_ROCKS;
        drop_rock(&mut tower, &mut iter_dirs, rock, &mut heights);
    }
    let height = heights.iter().max().unwrap();
    println!("{}", height);
}

fn part2(dirs: &Vec<Direction>, target: usize) {
    let target = target - 1;
    // who knows how high we have to iterate; this isn't even a multiple of the cycle length.
    let max_iters = dirs.len() * N_ROCKS;

    let mut history_map = HashMap::<Grid<bool>, HashMap<usize, (i64, usize)>>::new();

    // who knows how much we have to allocate; this would allow for every rock to be a vertical
    // line and land in a stack.
    let mut tower = Grid::<bool>::new(7, max_iters * 4 + 2);
    let mut heights = [0; 7];
    let mut iter_dirs = dirs.iter().enumerate().cycle().peekable();

    let mut cycle_end = 0;
    let mut cycle_length = 0;
    let mut height_diff = 0;

    // iterate until we find a cycle
    for i in 0..max_iters {
        let rock: usize = i % N_ROCKS;
        drop_rock(&mut tower, &mut iter_dirs, rock, &mut heights);

        // the grid history we need to remember is the current max height down to the minimum height
        // because everything below that is inaccessible.
        let min_height = max(0, *heights.iter().min().unwrap() - 1) as usize;
        let max_height = *heights.iter().max().unwrap() as usize;
        let history = tower.subgrid(Pt(0, min_height), Pt(7, max_height));
        let dir_i = iter_dirs.peek().unwrap().0;

        // Did we see this grid history at this point in the directions before?
        let map = history_map.entry(history).or_insert(HashMap::new());
        if let Some(&(old_height, j)) = map.get(&dir_i) {
            // yes! we found a cycle.
            cycle_end = i;
            cycle_length = i - j;
            height_diff = max_height as i64 - old_height;
            break;
        }
        // no - store the current iteration.
        map.insert(dir_i, (max_height as i64, i));
    }

    if cycle_length == 0 {
        panic!("Did not find cycle :(");
    }
    // Calculate how many cycles of the found length plus remainder it would take, after getting to
    // where we got to, to get up to the target.
    let cycles = (target - cycle_end) / cycle_length;
    let remaining = target - ((cycles * cycle_length) + cycle_end);

    let height_start = *heights.iter().max().unwrap();

    for i in 0..remaining {
        let rock: usize = (i + cycle_end + 1) % N_ROCKS;
        drop_rock(&mut tower, &mut iter_dirs, rock, &mut heights);
    }

    let height_remaining = *heights.iter().max().unwrap() - height_start;
    // Answer is the height reached by the first simulation, plus cycles * the height added on each
    // cycle, plus the height increase from the second simulation.
    println!(
        "{}",
        height_start + cycles as i64 * height_diff + height_remaining
    );
}

fn main() {
    let input = read_input().expect("Could not read input");
    let dirs = input
        .bytes()
        .into_iter()
        .filter_map(|c| match c as char {
            '<' => Some(Left),
            '>' => Some(Right),
            _ => None,
        })
        .collect::<Vec<_>>();

    // It's possible to use part2 code to solve part1, but the overhead of looking for cycles
    // makes it slower.
    part1(&dirs, 2022);
    part2(&dirs, 1000000000000);
}
