use std::error::Error;
use std::io::{self, Read};

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
    Ok(())
}
