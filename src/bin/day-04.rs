use std::ops::RangeInclusive;
use aoc2022::common::read_input_lines;

fn make_range(s: &str) -> RangeInclusive<u64> {
    let ints = s.split('-').map(|n| n.parse::<u64>().expect("Invalid integer")).collect::<Vec<_>>();
    if ints.len() != 2 {
        panic!("range {} has two many -s", s);
    }
    ints[0]..=ints[1]
}
fn make_ranges(strs: Vec<&str>) -> (RangeInclusive<u64>, RangeInclusive<u64>) {
    if strs.len() != 2 {
        panic!("line {:?} has length other than 2", strs);
    }
    (make_range(strs[0]), make_range(strs[1]))
}
fn main() {
    let lines = read_input_lines().expect("Could not read input");
    let rangepairs = lines.map(
        |line| make_ranges(line.split(',').collect::<Vec<_>>())
    ).collect::<Vec<_>>();
    println!("{}", rangepairs.iter().filter(
        |(a, b)|
            (a.start() >= b.start() && a.end() <= b.end()) ||
            (b.start() >= a.start() && b.end() <= a.end())
    ).count());
    println!("{}", rangepairs.iter().filter(
        |(a, b)|
            (a.start() <= b.start() && a.end() >= b.start()) ||
            (b.start() <= a.start() && b.end() >= a.start())
    ).count());
}
