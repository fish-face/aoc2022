use std::cmp::max;
use std::collections::VecDeque;
use ndarray::{arr1, Array, Array3, Dim, Dimension};
use aoc2022::common::read_input;
use anyhow::Result;

// making these functions generic takes 100 times as long as writing a function to just work
// on the types you need each puzzle :(
fn add(p: [usize; 3], q: [isize; 3]) -> [isize; 3] {
    [p[0] as isize + q[0], p[1] as isize + q[1], p[2] as isize + q[2]]
}

fn u(p: [isize; 3]) -> Result<[usize; 3]> {
    Ok([p[0].try_into()?, p[1].try_into()?, p[2].try_into()?])
}

fn lt(p: [usize; 3], q: (usize, usize, usize)) -> bool {
    p[0] < q.0 && p[1] < q.1 && p[2] < q.2
}

fn flood_fill(store: &mut Array<u8, Dim<[usize; 3]>>) {
    let mut queue = VecDeque::from([[0, 0, 0]]);
    // can't even be bothered re-looking up how to do lazy_static so this is here twice :) (: :) (:
    let neighbours = [
        [-1 as isize,  0,  0],
        [ 1,  0,  0],
        [ 0, -1,  0],
        [ 0,  1,  0],
        [ 0,  0, -1],
        [ 0,  0,  1],
    ];
    while queue.len() > 0 {
        let cur = queue.pop_front().unwrap();
        if store[cur] != 0 {
            continue;
        }
        // seen[cur] = 1;
        store[cur] = 2;
        for n in neighbours {
            if let Ok(q) = u(add(cur, n)) {
                if lt(q, store.dim()) {
                    queue.push_back(q);
                }
            }
        }
    }
}

fn count(store: &Array<u8, Dim<[usize; 3]>>, coords: &Vec<[usize; 3]>, target: u8) -> usize {
    let neighbours = [
        [-1 as isize,  0,  0],
        [ 1,  0,  0],
        [ 0, -1,  0],
        [ 0,  1,  0],
        [ 0,  0, -1],
        [ 0,  0,  1],
    ];
    let mut ct = 0;
    for p in coords {
        for n in &neighbours {
            if store[u(add(*p, *n)).unwrap()] == target {
                ct += 1;
            }
        }
    }
    ct
}

fn main() {
    let input = read_input().expect("Could not read input");
    let coords: Vec<[usize; 3]> = input.split('\n').filter(|line| line.len() > 0).map(|line|
        line.split(',').map(|n|
            n.parse::<usize>().expect("Invalid number") + 1
        ).collect::<Vec<_>>().try_into().unwrap()
    ).collect::<Vec<_>>();
    let bounds = coords.iter().fold(
        (0, 0, 0),
        |p, q| (max(p.0, q[0] + 2), max(p.1, q[1] + 2), max(p.2, q[2] + 2))
    );
    let mut store = Array::<u8, _>::zeros(bounds);
    for p in &coords {
        store[*p] = 1;
    }

    println!("{}", count(&store, &coords, 0));
    flood_fill(&mut store);
    println!("{}", count(&store, &coords, 2));
}
