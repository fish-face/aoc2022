use std::collections::HashSet;
use aoc2022::common::read_input_lines;

fn halve_lines(line: &String) -> (String, String) {
    let len = line.len();
    (line[..len / 2].to_string(), line[len / 2..].to_string())
}

fn priority(c: char) -> i64 {
    (match c {
        'a'..='z' => c as u8 - 'a' as u8 + 1,
        'A'..='Z' => c as u8 - 'A' as u8 + 27,
        _ => panic!("non-alphabetic character"),
    }) as i64
}

fn main() {
    let lines = read_input_lines().expect("Could not read input").collect::<Vec<_>>();
    let compartments = lines.iter().map(halve_lines);
    let intersections = compartments.map(
        |(a, b)|
            HashSet::<_>::from_iter(a.chars()).intersection(
                &HashSet::<_>::from_iter(b.chars())
            ).next().expect("There was a line with no intersection").clone()
    );
    println!("{}", intersections.map(priority).sum::<i64>());
    let badges = lines.chunks(3).map(
        |chunk: &[String]|
            if let Some((first, rest)) = chunk.split_first() {
                first
                    .chars()
                    .filter(|c| rest.iter().all(|other| other.contains(*c)))
                    .next()
                    .expect("No badge found")
            } else {
                panic!("Chunk of wrong size: {}", chunk.len())
            }
    );
    println!("{}", badges.map(priority).sum::<i64>());
}