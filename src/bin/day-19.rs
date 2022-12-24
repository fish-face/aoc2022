use std::cmp::{max, min};
use std::collections::{HashMap, VecDeque};
use aoc2022::common::read_input;
use anyhow::{Error, Result};

#[derive(Debug)]
pub struct Scenario {
    id: usize,
    ore_robot_ore: usize,
    clay_robot_ore: usize,
    obsidian_robot_ore: usize,
    obsidian_robot_clay: usize,
    geode_robot_ore: usize,
    geode_robot_obsidian: usize,
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
                ore_robot_ore: ore_robot,
                clay_robot_ore: clay_robot,
                obsidian_robot_ore: obs_robot_ore,
                obsidian_robot_clay: obs_robot_clay,
                geode_robot_ore: geode_robot_ore,
                geode_robot_obsidian: geode_robot_obsidian,
            }
        }
        rule _()
            = [' ' | '\n' | '\t']*
        rule num() -> usize
            = n:$(['0'..='9']+) {? n.parse().or(Err("bad number"))}
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct RobotState {
    ore_robots: usize,
    clay_robots: usize,
    obsidian_robots: usize,
    geode_robots: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    time: usize,
    robots: RobotState,
    ore: usize,
    clay: usize,
    obsidian: usize,
    geodes: usize,
}

impl State {
    fn new(ore_robots: usize) -> Self {
        State{
            time: 0,
            robots: RobotState{
                ore_robots,
                clay_robots: 0,
                obsidian_robots: 0,
                geode_robots: 0,
            },
            ore: 0,
            clay: 0,
            obsidian: 0,
            geodes: 0,
        }
    }
}

fn better(a: &State, b: &State) -> bool {
    a.time <= b.time &&
        // heuristic(a, 24) >= heuristic(b, 24)
        a.ore >= b.ore &&
        a.clay >= b.clay &&
        a.obsidian >= b.obsidian &&
        a.geodes >= b.geodes
}

type Heuristic = (usize);

fn heuristic(state: &State, max_time: usize) -> Heuristic {
    let t = max_time - state.time;
    (
        // state.time,
        state.ore + t * state.robots.ore_robots +
            state.clay + t * state.robots.clay_robots +
            state.obsidian + t * state.robots.obsidian_robots +
            state.geodes + t * state.robots.geode_robots
    )
}

fn advance(state: &mut State, t: usize) {
    state.time += t;
    state.ore = state.ore + state.robots.ore_robots * t;
    state.clay = state.clay + state.robots.clay_robots * t;
    state.obsidian = state.obsidian + state.robots.obsidian_robots * t;
    state.geodes = state.geodes + state.robots.geode_robots * t;
}

pub const fn div_ceil(lhs: usize, rhs: usize) -> Option<usize> {
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

fn build_ore(from: &State, scenario: &Scenario) -> Option<State> {
    let mut build = from.clone();
    let res_need = scenario.ore_robot_ore.saturating_sub(build.ore);
    let time = match res_need > 0 {
        true => div_ceil(res_need, build.robots.ore_robots)?,
        false => 1
    };
    advance(&mut build, time);
    build.ore -= scenario.ore_robot_ore;
    build.robots.ore_robots += 1;
    Some(build)
    // time.div_ceil();
}

fn build_clay(from: &State, scenario: &Scenario) -> Option<State> {
    let mut build = from.clone();
    let res_need = scenario.clay_robot_ore.saturating_sub(build.ore);
    let time = match res_need > 0 {
        true => div_ceil(res_need, build.robots.clay_robots)?,
        false => 1
    };
    advance(&mut build, time);
    build.ore -= scenario.clay_robot_ore;
    build.robots.clay_robots += 1;
    Some(build)
}

fn build_obsidian(from: &State, scenario: &Scenario) -> Option<State> {
    let mut build = from.clone();
    let res_need = scenario.obsidian_robot_ore.saturating_sub(build.ore);
    let time = match res_need > 0 {
        true => div_ceil(res_need, build.robots.ore_robots)?,
        false => 1
    };
    let res_need = scenario.obsidian_robot_clay.saturating_sub(build.clay);
    let time = max(time, match res_need > 0 {
        true => div_ceil(res_need, build.robots.clay_robots)?,
        false => 1
    });
    advance(&mut build, time);
    build.ore -= scenario.obsidian_robot_ore;
    build.clay -= scenario.obsidian_robot_clay;
    build.robots.obsidian_robots += 1;
    Some(build)
}

fn build_geode(from: &State, scenario: &Scenario) -> Option<State> {
    let mut build = from.clone();
    let res_need = scenario.geode_robot_ore.saturating_sub(build.ore);
    let time = match res_need > 0 {
        true => div_ceil(res_need, build.robots.ore_robots)?,
        false => 1
    };
    let res_need = scenario.geode_robot_obsidian.saturating_sub(build.obsidian);
    let time = max(time, match res_need > 0 {
        true => div_ceil(res_need, build.robots.obsidian_robots)?,
        false => 1
    });
    advance(&mut build, time);
    build.ore -= scenario.geode_robot_ore;
    build.obsidian -= scenario.geode_robot_obsidian;
    build.robots.geode_robots += 1;
    Some(build)
}

fn bfs_from(
    max_time: usize,
    start_state: State,
    scenario: &Scenario,
) -> usize {
    let mut queue:VecDeque<_> = VecDeque::<State>::new();
    let mut seen = HashMap::<RobotState, State>::new();
    let mut best = 0_usize;
    let mut count = 0_usize;

    queue.push_front(start_state);

    while queue.len() > 0 {
        let cur = queue.pop_front().unwrap();
        count += 1;
        if cur.time >= max_time {
            continue;
        }
        best = max(best, cur.geodes + (max_time - cur.time) * cur.robots.geode_robots);
        // let (th, hh) = heuristic(&cur, max_time);
        // match seen.get(&cur.robots) {
        //     Some((t, h)) => if *t >= th && *h >= hh {continue;}
        //     None => {}
        // }
        // seen.insert(cur.robots.clone(),heuristic(&cur, max_time));
        match seen.get(&cur.robots) {
            Some(t) => if better(t, &cur) {continue;}
            None => {}
        }
        seen.insert(cur.robots.clone(),cur.clone());

        // if cur in seen continue
        // insert cur nto seen

        let next = State{
            time: cur.time + 1,
            robots: cur.robots.clone(),
            ore: cur.ore + cur.robots.ore_robots,
            clay: cur.clay + cur.robots.clay_robots,
            obsidian: cur.obsidian + cur.robots.obsidian_robots,
            geodes: cur.geodes + cur.robots.geode_robots,
        };

        // For each type of robot, see if the *current* stock is enough to build it.
        // If so, clone the *next* state and subtract the stock, and add this to the queue.
        // This is because paying happens at the start of the phase, but you get the robot at the end.
        [build_ore, build_clay, build_obsidian, build_geode]
            .iter()
            .for_each(|f|
                if let Some(state) = f(&cur, scenario) {
                    queue.push_back(state)
                }
            );
        // queue.push_back(build_ore(&cur, scenario));
        // queue.push_back(build_clay(&cur, scenario));
        // queue.push_back(build_obsidian(&cur, scenario));
        // queue.push_back(build_geode(&cur, scenario));
        // if cur.ore >= scenario.ore_robot_ore {
        //     let mut build = next.clone();
        //     build.ore -= scenario.ore_robot_ore;
        //     build.robots.ore_robots += 1;
        //     queue.push_back(build);
        // }
        // if cur.ore >= scenario.clay_robot_ore {
        //     let mut build = next.clone();
        //     build.ore -= scenario.clay_robot_ore;
        //     build.robots.clay_robots += 1;
        //     queue.push_back(build);
        // }
        // if cur.ore >= scenario.obsidian_robot_ore && cur.clay >= scenario.obsidian_robot_clay {
        //     let mut build = next.clone();
        //     build.ore -= scenario.obsidian_robot_ore;
        //     build.clay -= scenario.obsidian_robot_clay;
        //     build.robots.obsidian_robots += 1;
        //     queue.push_back(build);
        // }
        // if cur.ore >= scenario.geode_robot_ore && cur.obsidian >= scenario.geode_robot_obsidian {
        //     let mut build = next.clone();
        //     build.ore -= scenario.geode_robot_ore;
        //     build.obsidian -= scenario.geode_robot_obsidian;
        //     build.robots.geode_robots += 1;
        //     queue.push_back(build);
        // }
        // queue.push_back(next);
    }
    println!("explored {} score {}", count, best);
    best
}

fn part1(scenarios: &Vec<Scenario>) {
    let score = scenarios
        .iter()
        .map(|scenario|
            bfs_from(
                24,
                State::new(1),
                scenario
            ) * scenario.id
        )
        .collect::<Vec<_>>();
    // .sum();
    println!("{:?}", score);
}

fn part2(scenarios: &Vec<Scenario>) {
    let score: usize = scenarios[0..max(2, scenarios.len() - 1)]
        .iter()
        .map(|scenario|
                 bfs_from(
                     32,
                     State::new(1),
                     scenario
                 ) * scenario.id
        )
        // .collect::<Vec<_>>();
        .product();
    println!("{:?}", score);
}

fn main() {
    let input = read_input().expect("Could not read input");
    let scenarios = parser::scenarios(&input).expect("Could not parse input");
    println!("{:?}", scenarios);
    part1(&scenarios);
    // part2(&scenarios);
}
