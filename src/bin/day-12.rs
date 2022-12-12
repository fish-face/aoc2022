use aoc2022::common::read_input_lines;
use aoc2022::coord::Pt;
use aoc2022::grid::Grid;
use petgraph::algo::dijkstra;
use petgraph::{Directed, Graph};

fn convert_ends(c: u8) -> u8 {
    match c as char {
        'S' => 'a' as u8,
        'E' => 'z' as u8,
        _ => c,
    }
}

fn main() {
    let input: Vec<_> = read_input_lines().expect("Could not read input").collect();
    let grid: Grid<u8> = Grid::from_data(
        input[0].len(),
        input.len(),
        input.join("").bytes().map(|x| x).collect::<Vec<_>>(),
    );

    let mut start = None;
    let mut end = None;
    let mut graph = Graph::<u8, (), Directed>::new();

    // Create graph nodes and store their indices somewhere
    let graph_grid = grid.map(|v: &u8| graph.add_node(*v));
    // Create graph edges, and also search for the start and end
    for (p, i) in graph_grid.enumerate() {
        let mut v = grid[p];
        if v == 'E' as u8 {
            end = Some(i);
        } else if v == 'S' as u8 {
            start = Some(i);
        }
        v = convert_ends(v);
        // AHAHAHAHA I LOVE RUST
        let p_isize = Pt(p.0 as isize, p.1 as isize);
        for neighbour in p_isize.neighbours4() {
            if neighbour.0 < 0 || neighbour.1 < 0 {
                continue;
            }
            // AHAHAHAHA I LOVE RUST
            let neighbour = Pt(neighbour.0 as usize, neighbour.1 as usize);
            if let (Ok(u), Ok(j)) = (grid.get(neighbour), graph_grid.get(neighbour)) {
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
            .filter(|(_, v)| **v == 'a' as u8)
            .map(|(p, _)| dijkstra_map.get(&graph_grid[p]).unwrap_or(&9001))
            .min()
            .expect("No route found")
    );
}
