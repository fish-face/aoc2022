extern crate core;

use std::collections::VecDeque;
use regex::Regex;
use aoc2022::common::read_input_lines;

fn main() {
    let mut lines = read_input_lines().expect("Could not read input");
    let stacks_str = lines.by_ref().take_while(|line| line != "").collect::<Vec<_>>();
    let n = (stacks_str[0].len() + 2) / 4;

    let mut stacks1:Vec<VecDeque<char>> = Vec::new();
    for _ in 0..n {
        stacks1.push(VecDeque::new());
        // stacks2.push(VecDeque::new());
    }
    for stack_line in &stacks_str[..stacks_str.len()-1] {
        for i in 0..n {
            let c = stack_line.as_bytes()[(i * 4) + 1] as char;
            if c != ' ' {
                stacks1[i].push_front(c);
                // stacks2[i].push_front(c);
            }
        }
    }
    let mut stacks2 = stacks1.clone();
    // println!("{:?}", stacks);
    let r = Regex::new(r"move ([0-9]+) from ([0-9]+) to ([0-9]+)").expect("Instruction regex did not compile");

    for instruction in lines {
        let captures = r
            .captures(instruction.as_str())
            .expect("Line did not parse")
            .iter()
            .skip(1)
            .map(|cap| cap.unwrap().as_str().parse::<usize>().unwrap_or_else(|v| panic!("{} {}", v, instruction)))
            .collect::<Vec<_>>();
        if let [count, from, to] = &captures[..] {
            // println!("{}", instruction);
            let fromstack = &mut stacks2[*from - 1];
            let len = fromstack.len();
            for v in fromstack.make_contiguous()[len - count - 1..len].iter() {
                stacks2[*to - 1].push_back(*v);
            }
            for _ in 0..*count {
                let v = stacks1[*from - 1].pop_back().expect("Stack was empty");
                stacks1[*to - 1].push_back(v);
                fromstack.pop_back().expect("Stack 2 was empty");
            }
        } else {
            panic!("wut");
        }
    }
    println!("{}", stacks1.iter().map(|stack| stack[stack.len() - 1].to_string()).collect::<Vec<_>>().join(""));
    println!("{}", stacks2.iter().map(|stack| stack[stack.len() - 1].to_string()).collect::<Vec<_>>().join(""));
}
