use aoc2022::common::read_input;
use aoc2022::coord::Pt;
use std::cmp::{max};

#[derive(Debug, Clone)]
pub struct Sensor {
    sensor: Pt<i64>,
    beacon_dist: i64,
}

peg::parser! {
    grammar parser() for str {
        pub rule input() -> Vec<Sensor>
            = v:(line() ** "\n") "\n"? {v}
        rule line() -> Sensor
            = "Sensor at x=" xs:num() ", y=" ys:num() ": closest beacon is at x=" xb:num() ", y=" yb:num() {
            Sensor{sensor: Pt(xs, ys), beacon_dist: (xs - xb).abs() + (ys - yb).abs()}
        }
        rule num() -> i64
            = n:$(['0'..='9' | '-']+) {? n.parse().or(Err("bad number"))}
    }
}

#[derive(Debug, Clone)]
enum Boundary {
    Start(i64),
    End(i64),
}

fn boundaries<'a>(y: i64, max_x: i64, sensors: impl Iterator<Item = &'a Sensor>) -> Vec<Boundary> {
    let mut boundaries = vec![];
    for sensor in sensors {
        let ydist = (sensor.sensor.1 - y).abs();
        let r = sensor.beacon_dist - ydist;
        if r > 0 {
            boundaries.push(Boundary::Start(sensor.sensor.0 - r));
            boundaries.push(Boundary::End(sensor.sensor.0 + r));
        }
    }
    boundaries.sort_by_key(|b| match b {
        Boundary::Start(x) | Boundary::End(x) => *x,
    });
    boundaries
}

fn main() {
    let input = read_input().expect("Could not read input");
    let mut sensors: Vec<Sensor> = parser::input(&input).unwrap();
    // Sort sensors by sensor x coordinate so that the boundaries start off closer to ordered
    sensors.sort_by_key(|s| s.sensor.0);

    // Values from the puzzle
    let max_xy = 4000000;
    let line = 2000000;

    // Part 1
    // get sorted boundaries of sensed ranges. The ranges will overlap.
    let bounds = boundaries(line, 200000000, sensors.iter());
    let mut depth = 0;
    let mut start = 0;
    let mut count = 0;
    for boundary in bounds.iter() {
        // Keep track of how many overlaps there are in depth
        match boundary {
            Boundary::Start(x) => {
                if depth == 0 {
                    // we are starting an unsensed region; record where it started
                    start = *x;
                }
                depth += 1;
            }
            Boundary::End(x) => {
                depth -= 1;
                if depth == 0 {
                    // we are finishing a sensed region; count how many cells it was
                    count += *x - start;
                }
            }
        }
    }
    println!("{}", count);

    // Part 2 - does the same as part 1 but for each y coordinate
    //  and instead of counting spaces, print the first real space.
    for y in 0..=max_xy {
        let mut depth = 0;
        let mut last = 0;
        let bounds = boundaries(y, max_xy, sensors.iter());
        for boundary in bounds.iter() {
            match boundary {
                Boundary::Start(x) => {
                    if depth == 0 && *x > last + 1 {
                        println!("{}", (last + 1) * 4000000 + y);
                        break;
                    }
                    depth += 1;
                }
                Boundary::End(x) => {
                    depth -= 1;
                    if depth == 0 {
                        last = *x;
                    }
                }
            }
        }
    }
}
