use aoc2022::common::read_input;
use std::cmp::{max, min, Ordering};
use std::collections::{HashMap, VecDeque};
use std::slice::Iter;

#[derive(Copy, Clone)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

use Resource::*;

impl Resource {
    pub fn iterator() -> Iter<'static, Resource> {
        static RESOURCES: [Resource; 4] = [Ore, Clay, Obsidian, Geode];
        RESOURCES.iter()
    }
}

type Res = [usize; 4];

#[derive(Debug)]
pub struct Scenario {
    id: usize,
    costs: [Res; 4],
}

peg::parser! {
    grammar parser() for str {
        pub rule scenarios() -> Vec<Scenario>
            = s:(scenario() ** "\n") "\n"? {s}
        rule scenario() -> Scenario
            = "Blueprint " id:num()
            ":" _ "Each ore robot costs " ore_robot:num()
            " ore." _ "Each clay robot costs " clay_robot:num()
            " ore." _ "Each obsidian robot costs " obs_robot_ore:num()
            " ore and " obs_robot_clay:num()
            " clay." _ "Each geode robot costs " geode_robot_ore:num()
            " ore and " geode_robot_obsidian:num()" obsidian." {
            Scenario{
                id: id,
                costs: [
                    [ore_robot, 0, 0, 0],
                    [clay_robot, 0, 0, 0],
                    [obs_robot_ore, obs_robot_clay, 0, 0],
                    [geode_robot_ore, 0, geode_robot_obsidian, 0],
                ]
            }
        }
        rule _()
            = [' ' | '\n' | '\t']*
        rule num() -> usize
            = n:$(['0'..='9']+) {? n.parse().or(Err("bad number"))}
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    time: usize,
    robots: Res,
    resources: Res,
}

impl State {
    fn new(ore_robots: usize) -> Self {
        State {
            time: 0,
            robots: [ore_robots, 0, 0, 0],
            resources: [0, 0, 0, 0],
        }
    }
}

impl PartialOrd<Self> for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.resources.cmp(&other.resources)
    }
}

fn advance(state: &mut State, t: usize) {
    state.time += t;
    state
        .resources
        .iter_mut()
        .zip(state.robots)
        .for_each(|(res, robots)| *res += robots * t)
}

// Why is this not a std function?
pub const fn checked_div_ceil(lhs: usize, rhs: usize) -> Option<usize> {
    if rhs == 0 {
        None
    } else {
        let d = lhs / rhs;
        let r = lhs % rhs;
        if r > 0 && rhs > 0 {
            Some(d + 1)
        } else {
            Some(d)
        }
    }
}

fn build_robot(from: &State, scenario: &Scenario, robot_kind: Resource) -> Option<State> {
    let mut build = from.clone();
    let robot_kind = robot_kind as usize;
    // The number of each type of resource needed to build the selected robot
    let res_need = scenario.costs[robot_kind]
        .iter()
        .zip(build.resources)
        .map(|(cost, res)| cost.saturating_sub(res));
    // The time needed to acquire each kind of resource, or short circuit and return None if
    // any required resource has no builders
    let time = res_need
        .zip(build.robots)
        .map(|(need, robots)| {
            Some(match need > 0 {
                // divide needed resource by builders, rounded up, or short circuit if no builders
                true => checked_div_ceil(need, robots)?,
                // if we don't need any, we don't need builders
                false => 0,
            })
        })
        .try_fold(0_usize, |prev, next| next.map(|ok| max(prev, ok)))?
        + 1;

    // Advance the output state by the required amount of time, acquiring resources
    advance(&mut build, time);
    // Subtract required resources from accumulated
    build
        .resources
        .iter_mut()
        .zip(scenario.costs[robot_kind])
        .for_each(|(res, robots)| *res -= robots);
    // Build robot
    build.robots[robot_kind] += 1;
    Some(build)
}

fn bfs_from(max_time: usize, start_state: State, scenario: &Scenario) -> usize {
    let mut queue: VecDeque<_> = VecDeque::<State>::new();
    let mut seen = HashMap::<Res, State>::new();
    let mut best = 0_usize;

    queue.push_front(start_state);

    while queue.len() > 0 {
        let cur = queue.pop_front().unwrap();
        if cur.time >= max_time {
            continue;
        }
        best = max(
            best,
            cur.resources[Geode as usize] + (max_time - cur.time) * cur.robots[Geode as usize],
        );

        match seen.get(&cur.robots) {
            Some(t) => {
                if *t <= cur {
                    continue;
                }
            }
            None => {}
        }
        seen.insert(cur.robots.clone(), cur.clone());

        // try to build each kind of robot; if it works put it on the queue
        Resource::iterator().copied().for_each(|f| {
            if let Some(state) = build_robot(&cur, scenario, f) {
                queue.push_back(state)
            }
        });
    }
    best
}

fn part1(scenarios: &Vec<Scenario>) {
    let score = scenarios
        .iter()
        .map(|scenario| bfs_from(24, State::new(1), scenario) * scenario.id)
        .sum::<usize>();
    println!("{:?}", score);
}

fn part2(scenarios: &Vec<Scenario>) {
    let score: usize = scenarios[0..min(3, scenarios.len() - 1)]
        .iter()
        .map(|scenario| bfs_from(32, State::new(1), scenario))
        .product();
    println!("{:?}", score);
}

fn main() {
    let input = read_input().expect("Could not read input");
    let scenarios = parser::scenarios(&input).expect("Could not parse input");
    part1(&scenarios);
    part2(&scenarios);
}
