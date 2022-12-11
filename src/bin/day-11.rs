use aoc2022::common::{read_input};

#[derive(Debug, Clone)]
pub struct Monkey {
    items: Vec<u64>,
    operation: Expr,
    test_divisor: u64,
    true_dest: MonkeyId,
    false_dest: MonkeyId,
    inspections: u64,
}

type MonkeyId = usize;

impl Monkey {
    // Returns a vec of destinations and values which end up there
    pub fn take_turn(&self, unworry: impl Fn(u64) -> u64) -> Vec<(MonkeyId, u64)> {
        self.items
            .iter()
            .map(|i| unworry(self.operation.eval(*i)))
            .map(|i| match (i % self.test_divisor) == 0 {
                true => (self.true_dest, i),
                false => (self.false_dest, i),
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number(u64),
    Variable(String),
    Operation(Op, Box<Expr>, Box<Expr>),
}

impl Expr {
    pub(crate) fn eval(&self, old_val: u64) -> u64 {
        match self {
            Expr::Number(x) => *x,
            Expr::Variable(_) => old_val,
            Expr::Operation(op, a, b) => match op {
                Op::Add => a.eval(old_val) + b.eval(old_val),
                Op::Mul => a.eval(old_val) * b.eval(old_val),
            }, // _ => panic!("Unbound variable")
        }
    }
}

#[derive(Debug, Clone)]
pub enum Op {
    Add,
    Mul,
}

peg::parser! {
    grammar monkey_parser() for str {
        pub rule monkey() -> Monkey
            = "Monkey " num() ":" "\n" _
                "Starting items: " items:(num() ** ", ") "\n" _
                "Operation: new = " op:expr() "\n" _
                "Test: divisible by " test:num() "\n" _
                "If true: throw to monkey " t:num() "\n" _
                "If false: throw to monkey " f:num() "\n"? {
            Monkey{items: items, operation: op, test_divisor: test, true_dest: t as MonkeyId, false_dest: f as MonkeyId, inspections: 0}
        }

        rule expr() -> Expr
            = op_expr() / val()
        rule op_expr() -> Expr
            = a:val() _ op:op() _ b:val() {Expr::Operation(op, Box::new(a), Box::new(b))}

        rule val() -> Expr
            = num_into_expr() / name_into_expr()
        rule op() -> Op
            = c:$(['+' | '*']) {? match c {"+" => Ok(Op::Add), "*" => Ok(Op::Mul), _ => Err("Bad op")}}
        rule num_into_expr() -> Expr
            = n:num() {Expr::Number(n)}
        rule name_into_expr() -> Expr
            = n:name() {Expr::Variable(n)}

        rule num() -> u64
            = n:$(['0'..='9']+) {? n.parse().or(Err("bad number"))}
        rule name() -> String
            = n:$"old" {n.to_string()}

        rule _()
            = quiet!{[' ' | '\t']+}
    }
}

fn round(monkeys: &mut Vec<Monkey>, unworry: impl Fn(u64) -> u64) {
    for i in 0..monkeys.len() {
        monkeys[i].inspections += monkeys[i].items.len() as u64;
        let monkey = monkeys[i].clone();
        let result = monkey.take_turn(&unworry);
        for (dest, value) in result {
            monkeys[dest as usize].items.push(value);
        }
        monkeys[i].items.clear();
    }
}

fn top_inspections(monkeys: &Vec<Monkey>) {
    let mut inspections = monkeys
        .iter()
        .enumerate()
        .map(|(i, monkey)| (i, monkey.inspections))
        .collect::<Vec<_>>();
    inspections.sort_by_key(|(_, x)| -(*x as i64));
    let top = inspections[0..2].iter().map(|x| x.1).collect::<Vec<_>>();

    println!("{}", top[0] * top[1]);
}

fn main() {
    let input = read_input().expect("Could not read input");
    let mut monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(|block| monkey_parser::monkey(block))
        .enumerate()
        .map(|(i, x)| x.unwrap_or_else(|e| panic!("Could not parse monkey {}: {}", i, e)))
        .collect();

    let mut monkeys1 = monkeys.clone();
    for _ in 0..20 {
        round(&mut monkeys1, |n| n / 3);
    }
    top_inspections(&monkeys1);

    let lcm: u64 = monkeys.iter().map(|x| x.test_divisor).product();
    for _ in 0..10000 {
        round(&mut monkeys, |n| n % lcm);
    }
    top_inspections(&monkeys);
}