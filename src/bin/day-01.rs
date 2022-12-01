use std::env;
use std::fs::File;
use std::io::Read;
use std::collections::BinaryHeap;

fn main() {
    let path = env::args().nth(1).expect("No input supplied!");
    let mut file = File::open(path).expect("Could not open");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Could not read file");

    let blocks = content.split("\n\n").map(
        |block| block.split("\n").map(|line| line.parse::<i32>().unwrap_or(0)).sum::<i32>()
    );
    let mut heap = blocks.collect::<BinaryHeap<_>>();
    println!("{}", heap.peek().unwrap());
    let (max1, max2, max3) = (heap.pop(), heap.pop(), heap.pop());
    println!("{}", max1.unwrap() + max2.unwrap() + max3.unwrap())
}
