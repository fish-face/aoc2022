use aoc2022::common::read_input;

fn all_different(v: Vec<u8>) -> bool {
    for (i, c) in v.iter().enumerate() {
        for d in v[i+1..].iter() {
            if c == d {
                return false;
            }
        }
    }
    true
}

fn find_first_all_different(input: &str, size: usize) -> usize {
    let mut seen = vec![0_u8; (size - 1)];
    for (i, c) in input.bytes().enumerate() {
        if i < size || !all_different(seen.iter().copied().chain([c]).collect::<Vec<_>>()) {
            let dest = i % (size - 1);
            seen[dest] = c;
        } else {
            return i+1;
        }
    }
    input.len()
}

fn main() {
    let input = read_input().expect("Could not read input");
    println!("{}", find_first_all_different(&input, 4));
    println!("{}", find_first_all_different(&input, 14));
}
