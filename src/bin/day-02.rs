use std::str::FromStr;

use anyhow::{Result};
use thiserror::Error;

use aoc2022::common::{parse_input_lines};

#[derive(Debug)]
struct Play {
    theirs: i16,
    ours: i16,
}

impl Play {
    fn outcome(theirs: i16, ours:i16) -> i16 {
        (ours - theirs).rem_euclid(3)
    }
    fn score(&self, ours: i16) -> i64 {
        let winscore: i64 = match Self::outcome(self.theirs, ours) {
            1 => 6,
            0 => 3,
            2 => 0,
            _ => panic!("{} and {} = {}", ours, self.theirs, (ours - self.theirs).rem_euclid(3)),
        };
        ours as i64 + 1 + winscore
    }
    fn score1(&self) -> i64 {
        self.score(self.ours)
    }
    fn ours2(&self) -> i16 {
        (self.theirs + self.ours - 1).rem_euclid(3)
    }
    fn score2(&self) -> i64 {
        self.score(self.ours2())
    }
}
#[derive(Debug, Error)]
enum PlayErr {
    #[error("Line does not have length 3")]
    LineLengthError
}

impl FromStr for Play {
    type Err = PlayErr;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.len() != 3 {
            return Err(Self::Err::LineLengthError)
        }
        Ok(Self {
            theirs: (input.chars().nth(0).unwrap() as i16) - 65_i16,
            ours: (input.chars().nth(2).unwrap() as i16) - 65_i16 - 23_i16,
        })
    }
}

fn main() {
    let plays: Vec<Play>= parse_input_lines().unwrap().collect();
    println!("{:?}", plays.iter().map(|play| play.score1()).sum::<i64>());
    println!("{:?}", plays.iter().map(|play| play.score2()).sum::<i64>());
}