use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read};

fn count_ways(curr: u64, input: Vec<u64>, memo: &mut HashMap<String, Option<u64>>) -> Option<u64> {
    //println!("{:?}", input);
    let hash = format!("{}: {:?}", curr, input);
    if let Some(res) = memo.get(&hash) {
        return *res;
    }
    if input.len() <= 1 {
        return Some(1);
    }
    let res = match input.get(0) {
        Some(adpt) => {
            let diff = adpt - curr;
            //println!("{}, {}", curr, diff);
            match diff {
                3 => match count_ways(*adpt, input[1..].to_vec(), memo) {
                    Some(n) => Some(n),
                    None => None,
                },
                1 | 2 => {
                    let with = match count_ways(*adpt, input[1..].to_vec(), memo) {
                        Some(n) => n,
                        None => 0,
                    };
                    let without = match count_ways(curr, input[1..].to_vec(), memo) {
                        Some(n) => n,
                        None => 0,
                    };
                    Some(with + without)
                }
                _ => None,
            }
        }

        None => Some(1),
    };
    memo.insert(hash, res);
    res
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let mut input = buffer
        .trim_end()
        .split("\n")
        .map(|n| n.parse::<u64>())
        .collect::<Result<Vec<u64>, _>>()?;
    input.sort();
    let mut dist: HashMap<u64, u64> = HashMap::new();
    let adapts = input.iter();
    let mut prev: u64 = 0;
    for adpt in adapts {
        let diff = adpt - prev;
        match dist.get(&diff) {
            Some(f) => dist.insert(diff, f + 1),
            None => dist.insert(diff, 1),
        };
        prev = *adpt;
    }
    println!("{:?}", dist);
    println!(
        "Solution: {}",
        dist.get(&1).unwrap() * (dist.get(&3).unwrap() + 1)
    );
    let memo = &mut HashMap::new();
    println!("Ways: {:?}", count_ways(0, input, memo));
    Ok(())
}
