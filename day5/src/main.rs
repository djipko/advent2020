use std::error::Error;
use std::io::{self, Read};
use string_error::*;

#[derive(Debug)]
struct BoardingPass {
    row: usize,
    col: usize,
}

fn parse_range(code: &str, s: usize, e: usize) -> Option<usize> {
    let mut start = s;
    let mut end = e;
    for step in code.chars() {
        match step {
            'F' | 'L' => end = start + (end - start) / 2,
            'B' | 'R' => start = start + (end - start + 1) / 2,
            _ => break
        };
        //println!("{}, {}", start, end)
    };
    if end == start {
        Some(end)
    } else {
        None
    }
}

impl BoardingPass {
    fn parse(input: &str) -> Option<BoardingPass> {
        let (row_code, col_code) = input.split_at(7);
        let row = parse_range(row_code, 0, 127)?;
        let col = parse_range(col_code, 0, 7)?;
        Some(BoardingPass { row, col })
    }
    
    fn seat_id(&self) -> usize {
        self.row * 8 + self.col
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer);
    let passes = buffer
        .trim_end()
        .split("\n")
        .map(|code| BoardingPass::parse(&code))
        .collect::<Option<Vec<BoardingPass>>>()
        .ok_or(new_err("Failed parsing"))?;
    println!("Max pass: {:?}", passes.iter().map(|pass| pass.seat_id()).max());
    Ok(())
}
