use std::collections::{HashMap, HashSet};
use aoc2022::common::read_input_lines;
use aoc2022::coord::{TCoord};

fn main() {
    let input = read_input_lines().expect("Could not read input").collect::<Vec<_>>();

    let mut nodes: Vec<TCoord<i32>> = vec![TCoord::default(); 10];

    let direction = HashMap::from([
        ('R', TCoord( 1_i32,  0)),
        ('U', TCoord( 0,  1)),
        ('L', TCoord(-1,  0)),
        ('D', TCoord( 0, -1)),
    ]);

    let mut visited1: HashSet<TCoord<i32>> = HashSet::new();
    let mut visited_last: HashSet<TCoord<i32>> = HashSet::new();

    for line in input {
        let (dir_chars, dist_chars) = line.split_at(2);
        let dir_char = dir_chars.bytes().nth(0).unwrap() as char;

        let top_dir = *direction.get(&dir_char).unwrap_or_else(|| panic!("Didn't find {}", line));
        let dist = dist_chars.parse::<i32>().unwrap() as i32;

        for _ in 0..dist {
            nodes[0] = nodes[0] + top_dir;
            let mut dir = top_dir;
            for i in 1..nodes.len() {
                let head = nodes[i-1];
                let tail = nodes[i];
                let would_offset = head - tail;
                if would_offset.0.abs() > 1 && would_offset.1.abs() > 1 {
                    nodes[i] = nodes[i] + dir;
                } else if would_offset.0.abs() > 1 {
                    nodes[i].0 += dir.0;
                    nodes[i].1 = head.1;
                } else if would_offset.1.abs() > 1 {
                    nodes[i].1 += dir.1;
                    nodes[i].0 = head.0;
                }
                dir = nodes[i] - tail;
            }
            visited1.insert(nodes[1]);
            visited_last.insert(nodes[nodes.len() - 1]);
        }
    }
    println!("{}", visited1.len());
    println!("{}", visited_last.len());
}
