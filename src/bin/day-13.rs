use aoc2022::common::read_input;
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};

#[derive(PartialEq, Ord, Clone)]
pub struct List {
    children: Vec<ListVal>,
}

impl Debug for List {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.children.fmt(f)
    }
}

impl Eq for List {}

impl PartialOrd<Self> for List {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Vec::partial_cmp(&self.children, &other.children)
    }
}

#[derive(PartialEq, Eq, Ord, Clone)]
enum ListVal {
    Val(u8),
    List(List),
}

impl PartialOrd<Self> for ListVal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Val(v), Self::Val(u)) => u8::partial_cmp(v, u),
            (Self::Val(v), Self::List(u)) => List::partial_cmp(
                &List {
                    children: vec![Self::Val(*v)],
                },
                u,
            ),
            (Self::List(v), Self::Val(u)) => List::partial_cmp(
                v,
                &List {
                    children: vec![Self::Val(*u)],
                },
            ),
            (Self::List(v), Self::List(u)) => List::partial_cmp(v, u),
        }
    }
}

impl Debug for ListVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Val(v) => v.fmt(f),
            Self::List(l) => l.fmt(f),
        }
    }
}

peg::parser! {
    grammar pair_parser() for str {
        pub rule list_pair() -> (List, List)
            = a:list() "\n" b:list() "\n"? {(a, b)}

        rule list() -> List
            = "[" v:list_val() ** "," "]" {List{children: v}}

        rule list_val() -> ListVal
            = v:val_int() / v:val_list() {v}

        rule val_int() -> ListVal
            = v:num() {ListVal::Val(v)}
        rule val_list() -> ListVal
            = v:list() {ListVal::List(v)}

        rule num() -> u8
            = n:$(['0'..='9']+) {? n.parse().or(Err("bad number"))}
    }
}

fn main() {
    let input = read_input().expect("Could not read input");
    let mut pairs: Vec<(List, List)> = input
        .split("\n\n")
        .map(|block| pair_parser::list_pair(block))
        .enumerate()
        .map(|(i, x)| x.unwrap_or_else(|e| panic!("Could not parse pair {}: {}", i, e)))
        .collect();

    println!(
        "{:?}",
        pairs
            .iter()
            .enumerate()
            .filter(|(_, (a, b))| a < b)
            .map(|(i, _)| i + 1)
            .sum::<usize>()
    );

    let divider = pair_parser::list_pair(&"[[2]]\n[[6]]").unwrap();
    pairs.append(&mut vec![divider.clone()]);
    // can't flatten tuples :))))))))))))))
    let mut pairs = pairs.iter().map(|(a, b)| vec![a, b]).flatten().collect::<Vec<_>>();
    pairs.sort();
    let pos1 = pairs.iter().position(|l| **l == divider.0).expect("divider 1 disappeared") + 1;
    let pos2 = pairs.iter().position(|l| **l == divider.1).expect("divider 1 disappeared") + 1;
    println!("{}", pos1 * pos2);
}
