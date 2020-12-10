use std::error::Error;
use std::collections::HashMap;
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let mut input = buffer
        .trim_end()
        .split("\n")
        .map(|n| n.parse::<u32>())
        .collect::<Result<Vec<u32>, _>>()?;
    input.sort();
    let mut dist: HashMap<u32, u32> = HashMap::new();
    let mut adapts = input.iter();
    let mut prev: u32 = 0;
    for adpt in adapts {
        let diff = adpt - prev;
        match dist.get(&diff) {
            Some(f) => dist.insert(diff, f + 1),
            None => dist.insert(diff, 1),
        };
        prev = *adpt;

    }
    println!("{:?}", dist);
    println!("Solution: {}", dist.get(&1).unwrap() * (dist.get(&3).unwrap() + 1));
    Ok(())
}
