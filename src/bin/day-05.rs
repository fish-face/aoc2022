extern crate core;

use std::collections::VecDeque;
use regex::Regex;
use aoc2022::common::read_input_lines;

fn move_crates(stacks: &mut Vec<VecDeque<char>>, from: usize, to: usize, count: usize, reverse: bool) {
    let fromstack = &mut stacks[from];
    let mut moving = fromstack.split_off(fromstack.len() - count);
    if reverse {
        for v in moving.make_contiguous().iter().rev() {
            stacks[to].push_back(*v);
        }
    } else {
        stacks[to].append(&mut moving);
    }
}

fn main() {
    let mut lines = read_input_lines().expect("Could not read input");
    let stacks_str = lines.by_ref().take_while(|line| line != "").collect::<Vec<_>>();
    let n = (stacks_str[0].len() + 2) / 4;

    let mut stacks1:Vec<VecDeque<char>> = Vec::new();
    for _ in 0..n {
        stacks1.push(VecDeque::new());
    }
    for stack_line in &stacks_str[..stacks_str.len()-1] {
        for i in 0..n {
            let c = stack_line.as_bytes()[(i * 4) + 1] as char;
            if c != ' ' {
                stacks1[i].push_front(c);
            }
        }
    }
    let mut stacks2 = stacks1.clone();
    let r = Regex::new(r"move ([0-9]+) from ([0-9]+) to ([0-9]+)").expect("Instruction regex did not compile");

    for instruction in lines {
        let captures = r
            .captures(instruction.as_str())
            .expect("Line did not parse")
            .iter()
            .skip(1)
            .map(|cap|
                cap.unwrap().as_str().parse::<usize>().unwrap_or_else(|v| panic!("{} {}", v, instruction))
            )
            .collect::<Vec<_>>();
        if let [count, from, to] = &captures[..] {
            move_crates(&mut stacks1, *from - 1, *to - 1, *count, true);
            move_crates(&mut stacks2, *from - 1, *to - 1, *count, false);
        } else {
            panic!("wut");
        }
    }
    fn format_output(stacks: Vec<VecDeque<char>>) {
        println!("{}", stacks.iter().map(|stack| stack[stack.len() - 1].to_string()).collect::<Vec<_>>().join(""));
    }
    format_output(stacks1);
    format_output(stacks2);
}
