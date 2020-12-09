use combinations::Combinations;
use std::collections::{VecDeque, HashSet};
use std::error::Error;
use std::io::{self, Read};

fn valid_next(input: Vec<u64>) -> HashSet<u64> {
    Combinations::new(input, 2).map(|c| c.iter().sum()).collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let input = buffer.trim_end().split("\n").map(
        |n| n.parse::<u64>()).collect::<Result<Vec<u64>, _>>()?;
    let mut current: VecDeque<u64> = input.iter().cloned().take(25).collect();
    for &next in input[25..].iter() {
        if !valid_next(current.iter().cloned().collect()).contains(&next) {
            println!("Found: {}", next);
            break;
        }
        current.pop_back();
        current.push_front(next);
    }

    Ok(())
}
