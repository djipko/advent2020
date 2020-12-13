use std::error::Error;
use std::io::{self, Read};

#[derive(Debug)]
struct Departure {
    id: u64,
    wait: u64,
}

impl Departure {
    fn wait_for_ts(&self, ts: u64) -> u64 {
        (self.id - ts % self.id) % self.id
    }
}

fn matches(ts: u64, deps: &Vec<Departure>) -> bool {
    //println!("{}", ts);
    deps.iter().all(|d| {
        //println!("{:?}, {}", d, d.wait_for_ts(ts));
        d.wait_for_ts(ts) == d.wait
    })
}

fn matches_for(ts: u64, d: &Departure) -> bool {
    d.wait_for_ts(ts) == d.wait
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let data = buffer
        .trim_end()
        .split("\n").collect::<Vec<&str>>();
    let ts = data[0].parse::<i64>()?;
    let mut ids = data[1].split(",").filter(|id| *id != "x").map(|id| id.parse::<i64>())
        .collect::<Result<Vec<i64>, _>>()?;
    ids.sort_by_key(|id| id - ts % id);
    let wait = ids[0] - ts % ids[0];
    println!("id: {}, diff: {}, Res: {}", ids[0], wait, ids[0] * wait);
    let deps = data[1].split(",").enumerate().filter(|(_, id)| *id != "x").map(|(wait, id)| {
        let id = id.parse::<u64>().ok()?;
        let wait = wait as u64;
        if wait <= id {
            Some(Departure{id, wait})
        } else {
            Some(Departure{id, wait: wait % id})
        }
    }).collect::<Option<Vec<Departure>>>().ok_or("Failed parsing")?;
    println!("Deps: {:?}", deps);
    let start: u64 = 100000000000009;
    //let start: u64 = deps[0].id;

    let max = deps.iter().max_by_key(|d| d.id).unwrap();
    let mut first_max = start;
    for ts in (start..).step_by(deps[0].id as usize) {
        if matches_for(ts, &max) {
            println!("Max found: {}, {:?}", ts, max);
            first_max = ts;
            break;
        }
    }
    for ts in (first_max..).step_by((max.id * deps[0].id) as usize) {
        if matches(ts, &deps) {
            println!("Ts: {}", ts);
            break;
        }
    }
    Ok(())
}
