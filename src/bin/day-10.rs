use aoc2022::common::read_input_lines;
use peg::parser;
use aoc2022::coord::Pt;
use aoc2022::grid::Grid;

#[derive(Debug)]
pub enum Instruction {
    Noop,
    AddX(i32),
}

parser! {
    grammar instruction_parser() for str {
        rule num() -> i32
            = n:$(['0'..='9' | '-']+) {? n.parse().or(Err("bad number"))}
        rule noop() -> Instruction
            = "noop" {Instruction::Noop}
        rule addx() -> Instruction
            = "addx " x:num() {Instruction::AddX(x)}
        pub rule instruction() -> Instruction
            = noop() / addx()
    }
}

fn main() {
    let input = read_input_lines()
        .expect("Could not read input")
        .map(|line| instruction_parser::instruction(&line));

    let mut reg = 1;
    let mut cycle = 0;
    let mut acc = 0;
    let mut screen: Grid<u8> = Grid::new(40, 6);
    for instruction in input {
        let (cycles, x) = match instruction.unwrap() {
            Instruction::AddX(x) => {
                (2, x)
            },
            _ => {(1, 0)},
        };
        for i in 0..cycles {
            let (col, row) = (cycle % 40, cycle / 40);
            cycle += 1;

            // Count part 1 - offset by 1 due to calculating col before incrementing cycle
            if col == 19 {
                acc += (cycle) as i32 * reg;
            }

            // Do part 2
            if (reg - col).abs() <= 1 {
                screen[Pt(col as usize, row as usize)] = 1;
            }

            if i == cycles - 1 {
                reg += x;
            }
        }
    }
    println!("{}", acc);
    println!("{}", screen.map(|i| match *i {0 => '.', 1 => '#', _ => '_'}).to_string(Some("")));
}
