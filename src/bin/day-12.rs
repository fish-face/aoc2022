use petgraph::{Directed, Graph};
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, UnGraph};
use aoc2022::common::read_input_lines;
use aoc2022::coord::Pt;
use aoc2022::grid::Grid;

type Coord = Pt<usize>;

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

    let graph_grid = grid.map(|v: &u8| graph.add_node(*v));
    for (p, i) in graph_grid.enumerate() {
        let mut v = grid[p];
        if v == 'E' as u8 {
            end = Some(i);
        } else if v == 'S' as u8 {
            start = Some(i);
        }
        v = convert_ends(v);
        // println!("{} {}: {}", x, y, v);
        // AHAHAHAHA I LOVE RUST
        let p_isize = Pt(p.0 as isize, p.1 as isize);
        for neighbour in p_isize.neighbours4() {
            if neighbour.0 < 0 || neighbour.1 < 0 {continue;}
            // AHAHAHAHA I LOVE RUST
            let neighbour = Pt(neighbour.0 as usize, neighbour.1 as usize);
            if let (Ok(u), Ok(j)) = (grid.get(neighbour), graph_grid.get(neighbour)) {
                let u = convert_ends(*u);
                if v + 1 >= u {
                    graph.add_edge(*i, *j, ());
                }
            }
        }
    }
    let start = start.unwrap();
    let end = end.unwrap();
    // println!("{:?} {:?}", start, end);
    // println!("{}", grid.to_string(None));
    // println!("{}", graph_grid.map(|i| i.index()).to_string(None));
    // println!("{:?}", graph);
    let dijkstra_map = dijkstra(&graph, *start, Some(*end), |_| 1);
    // println!("{:#?}", dijkstra_map.iter().map(|(k, v)| format!("{} {}", k.index(), v)).collect::<Vec<_>>());
    println!("{:?}", dijkstra_map.get(end).expect("No route found"));
    // println!("{}", grid.to_string(Some(" ")))
}
