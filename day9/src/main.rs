use combinations::Combinations;
use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::io::{self, Read};

fn valid_next(input: Vec<&u64>) -> HashSet<u64> {
    Combinations::new(input, 2)
        .map(|c| c.iter().cloned().sum())
        .collect()
}

fn find_weak(input: &Vec<u64>) -> Option<u64> {
    let mut current: VecDeque<u64> = input.iter().cloned().take(25).collect();
    for &next in input[25..].iter() {
        if !valid_next(current.iter().collect()).contains(&next) {
            return Some(next);
        }
        current.pop_back();
        current.push_front(next);
    }
    None
}

fn enc_weakness(input: &Vec<u64>, weak: u64) -> Option<u64> {
    for n in 2..(input.len()) {
        let mut cur: VecDeque<u64> = input.iter().take(n).cloned().collect();
        let mut sum: u64 = cur.iter().sum();
        for &next in input[n..].iter() {
            //println!("{}: {:?}", sum, cur);
            if sum == weak {
                return Some(*cur.iter().min().unwrap() + *cur.iter().max().unwrap());
            }
            sum -= cur.pop_front().unwrap();
            sum += next;
            cur.push_back(next);
        }
    }
    None
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let input = buffer
        .trim_end()
        .split("\n")
        .map(|n| n.parse::<u64>())
        .collect::<Result<Vec<u64>, _>>()?;
    if let Some(weak) = find_weak(&input) {
        println!("Found: {}", weak);
        if let Some(enc_weak) = enc_weakness(&input, weak) {
            println!("Enc weakness: {}", enc_weak);
        }
    }

    Ok(())
}
