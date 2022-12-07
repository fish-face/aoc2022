use std::str;
use slab_tree::{TreeBuilder};

use aoc2022::common::{read_input_lines};

#[derive(Debug)]
pub struct File {
    size: usize,
}
type Dir = File;

#[derive(Debug)]
pub enum LogLine {
    Cd(String),
    Ls(),
    LsResult(Dir),
}

peg::parser!{
    grammar log_parser() for str {
        rule path() -> String
            = p:$(['a'..='z' | '.' | '/']+) {
                p.to_string()
            }
        rule num() -> usize
            = n:$(['0'..='9']+) {? n.parse().or(Err("bad number"))}
        rule lsdir() -> LogLine
            = "dir " path() {
                LogLine::LsResult(Dir{size: 0})
            }
        rule lsfile() -> LogLine
            = size:num() " " name:path() {
                LogLine::LsResult(File{size: size})
            }
        rule lsresult() -> LogLine
            = lsfile() / lsdir()
        rule ls() -> LogLine
            = "$ ls" {
                LogLine::Ls()
            }
        rule cd() -> LogLine
            = "$ cd " p:path() {
                LogLine::Cd(p.to_string())
            }
        pub rule log_line() -> LogLine
            = (cd() / ls() / lsresult())
    }
}

fn main() {
    let lines = read_input_lines().expect("Could not read input");
    let mut fs = TreeBuilder::new().with_root(Dir{size: 0}).build();
    let root_id = fs.root_id().unwrap();
    let mut cwd_id = root_id;

    for line in lines {
        let mut cwd = fs.get_mut(cwd_id).unwrap();
        if let Some(parsed) = log_parser::log_line(line.as_ref()).ok() {
            match parsed {
                LogLine::Cd(path) => {
                    match path.as_str() {
                        "/" => cwd_id = root_id,
                        ".." => {
                            let size = cwd.data().size;
                            let mut parent = cwd.parent().expect("Current directory has no parent!");
                            parent.data().size = parent.data().size + size;
                            cwd_id = parent.node_id()
                        },
                        _ => {
                            let dir = Dir{size: 0};
                            let newnode = cwd.append(dir);
                            cwd_id = newnode.node_id();
                        }
                    }
                }
                LogLine::LsResult(filedir) => {
                    match filedir {
                        File{size} => {
                            cwd.data().size = cwd.data().size + size;
                        },
                    }
                }
                _ => {},
            }
        } else {
            panic!("Aaaah {}", line.as_str());
        }
    }

    while cwd_id != root_id {
        let mut cwd = fs.get_mut(cwd_id).unwrap();
        let size = cwd.data().size;
        let mut parent = cwd.parent().unwrap();
        parent.data().size = parent.data().size + size;
        cwd_id = parent.node_id()
    }

    let mut sizes: Vec<usize> = fs.root().unwrap().traverse_post_order().map(|node| node.data().size).collect::<Vec<_>>();
    sizes.sort();
    println!("{}", sizes.iter().take_while(|size| **size < 100000).sum::<usize>());

    let target = fs.root().unwrap().data().size - 40000000;
    println!("{}", sizes.iter().find(|size| **size >= target).expect("No answer??"));
}
