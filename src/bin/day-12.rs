use aoc2022::common::read_input_lines;
use aoc2022::coord::Pt;
use aoc2022::grid::Grid;
use petgraph::algo::dijkstra;
use petgraph::{Directed, Graph};
use petgraph::graph::NodeIndex;

fn convert_ends(c: u8) -> u8 {
    match c as char {
        'S' => 'a' as u8,
        'E' => 'z' as u8,
        _ => c,
    }
}

fn main() {
    let input: Vec<_> = read_input_lines().expect("Could not read input").collect();
    let mut graph = Graph::<(), (), Directed>::new();

    // Create graph nodes and store their indices somewhere
    let grid: Grid<(u8, NodeIndex)> = Grid::from_data(
        input[0].len(),
        input.len(),
        input
            .join("")
            .bytes()
            .map(|x| (x, graph.add_node(())))
            .collect::<Vec<_>>(),
    );

    let mut start = None;
    let mut end = None;
    // Create graph edges, and also search for the start and end
    for (p, (v, i)) in grid.enumerate() {
        // let mut v = grid[p];
        if *v == 'E' as u8 {
            end = Some(i);
        } else if *v == 'S' as u8 {
            start = Some(i);
        }
        let v = convert_ends(*v);
        let p_isize = Pt::<isize>::from(p);
        for neighbour in p_isize.neighbours4() {
            if neighbour.0 < 0 || neighbour.1 < 0 {
                continue;
            }
            let neighbour = Pt::<usize>::from(neighbour);
            if let Ok((u, j)) = grid.get(neighbour) {
                let u = convert_ends(*u);
                // This condition is backwards because for part2 we have to search from the goal to several different places
                if u + 1 >= v {
                    graph.add_edge(*i, *j, ());
                }
            }
        }
    }
    let start = start.unwrap();
    let end = end.unwrap();
    // Search from the goal backwards so that we can find any valid starting position in part 2
    let dijkstra_map = dijkstra(&graph, *end, None, |_| 1);
    println!("{:?}", dijkstra_map.get(start).expect("No route found"));
    println!(
        "{:?}",
        grid.enumerate()
            .filter(|(_, (v, _))| *v == 'a' as u8)
            .map(|(p, _)| dijkstra_map.get(&grid[p].1).unwrap_or(&9001))
            .min()
            .expect("No route found")
    );
}
