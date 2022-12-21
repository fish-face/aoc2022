use aoc2022::common::read_input;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use array_init::array_init;

use bitvec::prelude::*;

type NodeID = u8;

#[derive(Clone)]
pub struct Valve {
    name: String,
    rate: i64,
    conn_names: Vec<String>,
    connections: Vec<Edge>,
}

#[derive(Clone, Debug)]
pub struct Edge {
    to: Rc<RefCell<Valve>>,
    cost: i64,
}

impl Debug for Valve {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Valve")
            .field("name", &self.name)
            .field("rate", &self.rate)
            .field("connections", &self.conn_names)
            .finish()
    }
}

peg::parser! {
    grammar parser() for str {
        pub rule input() -> Vec<Rc<RefCell<Valve>>>
            = v:(line() ** "\n") "\n"? {v}
        rule line() -> Rc<RefCell<Valve>>
            = "Valve " v:name()
              " has flow rate=" r:num()
              "; tunnel" "s"? " lead" "s"? " to valve" "s"? " " t:(name() ** ", ") {
            Rc::new(RefCell::new(Valve{name: v, rate: r, conn_names: t, connections: Vec::new()}))
        }
        rule name() -> String
            = s:$(['A'..='Z'] ['A'..='Z']) {s.to_owned()}
        rule num() -> i64
            = n:$(['0'..='9']+) {? n.parse().or(Err("bad number"))}
    }
}

fn dijkstra_map(nodes: impl Iterator<Item = Rc<RefCell<Valve>>>) -> HashMap<(String, String), i64> {
    let mut dist = HashMap::<(String, String), i64>::new();
    for start in nodes {
        let mut queue = VecDeque::<(Rc<RefCell<Valve>>, i64)>::new();
        let mut visited = HashSet::<String>::new();

        let start_name = RefCell::borrow(&start.borrow()).name.clone();
        queue.push_front((start, 0));

        while queue.len() > 0 {
            let (rccur, curdist) = queue.pop_front().unwrap();
            let cur = RefCell::borrow(rccur.borrow());
            if visited.contains(&cur.name.clone()) {
                continue;
            }
            dist.entry((start_name.clone(), cur.name.clone()))
                .and_modify(|d| *d = min(*d, curdist))
                .or_insert(curdist);
            visited.insert(cur.name.clone());
            for edge in cur.connections.iter() {
                let to = Rc::clone(&edge.to);
                let cost = edge.cost;

                queue.push_back((to, curdist + cost));
            }
        }
    }
    dist
}

type Valves = BitArr!(for 64);

struct SearchState {
    at: NodeID,
    time: i64,
    pressure: i64,
    active: Valves,
}

type NodeMap = Vec<(NodeID, i64)>;
type Graph = [NodeMap; 64];

fn part1(
    map: &Graph,
) {
    println!("{}", bfs_from(map, 30, SearchState {at: 0, time: 0, pressure: 0, active: bitarr![0]}, 0)) ;
}

fn part2(
    map: &Graph
) {
   println!("{}", bfs_from(map, 26, SearchState {at: 0, time: 0, pressure: 0, active: bitarr![0]}, 1)) ;
}

fn bfs_from(
    map: &Graph,
    max_time: i64,
    start_state: SearchState,
    remaining_agents: i64,
) -> i64 {
    let mut queue = VecDeque::<SearchState>::new();
    let mut seen = HashMap::<(NodeID, i64), i64>::new();
    let mut best = 0_i64;

    queue.push_front(start_state);

    while queue.len() > 0 {
        let cur = queue.pop_front().unwrap();
        if remaining_agents == 0 {
            best = max(best, cur.pressure);
        }
        // If we have been at the current node before, with the same valves activated, we just did something
        // pointless; cease this line of inquiry!
        if *seen.get(&(cur.at, cur.pressure)).unwrap_or(&i64::MAX) <= cur.time {
            continue;
        }
        seen.insert((cur.at, cur.pressure), cur.time);
        // if (vec!["AA".to_string(), "DD".into(), "HH".into(), "EE".into()].starts_with(&cur_state.hist_a) &&
        //    vec!["AA".to_string(), "JJ".into(), "BB".into(), "CC".into()].starts_with(&cur_state.hist_b)) ||
        //    (vec!["AA".to_string(), "DD".into(), "HH".into(), "EE".into()].starts_with(&cur_state.hist_b) &&
        //    vec!["AA".to_string(), "JJ".into(), "BB".into(), "CC".into()].starts_with(&cur_state.hist_a)) {
        //     println!("{:?}", cur_state);
        //     continue;
        // }

        // move A
        for (next, cost) in map[cur.at as usize].iter() {
            let rate = *next;
            if cur.time + cost + 1 <= max_time && !cur.active.get(rate as usize).expect("Index OOB") {
                let mut new_active_valves = cur.active.clone();
                new_active_valves.set(rate as usize, true);
                queue.push_back(SearchState {
                    at: *next,
                    time: cur.time + cost + 1,
                    pressure: cur.pressure + rate as i64 * (max_time - cur.time - cost - 1),
                    active: new_active_valves,
                })
            }
        }

        if cur.time > 5 && remaining_agents > 0 {
            // Move to start node and rewind time and try again
            let start = SearchState {
                at: 0,
                time: 0,
                pressure: cur.pressure,
                active: cur.active.clone(),
            };
            best = max(best, bfs_from(
                map,
                max_time,
                start,
                remaining_agents - 1,
            ))
        }
    }
    best
}

fn main() {
    let input = read_input().expect("Could not read input");
    let valves: Vec<Rc<RefCell<Valve>>> = parser::input(&input).unwrap();
    let lookup = valves
        .into_iter()
        .map(|valve| {
            (
                RefCell::<Valve>::borrow(valve.borrow()).name.clone(),
                Rc::clone(&valve),
            )
        })
        .collect::<HashMap<_, _>>();
    lookup.iter().for_each(|(_, valve)| {
        let connected = RefCell::<Valve>::borrow(valve.borrow())
            .conn_names
            .iter()
            .map(|n| Edge {
                to: Rc::clone(lookup.get(n).unwrap()),
                cost: 1,
            })
            .collect::<Vec<_>>();
        valve.borrow_mut().connections = connected;
    });
    let dm = dijkstra_map(lookup.iter().map(|(_, value)| value).cloned());
    // Construct an adjacency map graph using the *rates* as IDs (they seem to be unique)
    let mut dm_filtered: Graph = array_init(|_| Vec::new());
    for ((from, to), cost) in dm {
        let from_rate = RefCell::borrow(lookup[&from].borrow()).rate as NodeID;
        let to_rate = RefCell::borrow(lookup[&to].borrow()).rate as NodeID;
        if to_rate > 0 && (from == "AA" || from_rate > 0)
        {
            dm_filtered[from_rate as usize].push((to_rate, cost));
        }
    }
    part1(&dm_filtered);
    part2(&dm_filtered);
}
