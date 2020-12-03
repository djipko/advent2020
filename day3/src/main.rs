use std::error::Error;
use std::io::{self, Read};
use string_error::*;

#[derive(Debug)]
enum Field {
    Tree,
    Open,
}

#[derive(Debug)]
struct Grid {
    fields: Vec<Vec<Field>>,
    w: usize,
    h: usize,
    pos: (usize, usize),
}

impl Grid {
    fn parse(input: Vec<&str>) -> Option<Grid> {
        let h = input.len();
        let fields = input
            .iter()
            .map(|row| {
                row.chars()
                    .map(|cell| {
                        if cell == '#' {
                            Field::Tree
                        } else {
                            Field::Open
                        }
                    })
                    .collect::<Vec<Field>>()
            })
            .collect::<Vec<Vec<Field>>>();
        let w = fields.first()?.len();
        Some(Grid {
            fields,
            w,
            h,
            pos: (0, 0),
        })
    }

    fn step(&mut self, right: usize, down: usize) -> Option<&Field> {
        let new_x = (self.pos.0 + right) % self.w;
        let new_y = self.pos.1 + down;
        if new_y >= self.h {
            None
        } else {
            self.pos = (new_x, new_y);
            let stepped = self.fields.get(new_y)?.get(new_x)?;
            Some(stepped)
        }
    }

    fn loop_grid(&mut self, right: usize, down: usize) -> u32 {
        let mut tree_cnt = 0;
        loop {
            match self.step(right, down) {
                Some(Field::Tree) => tree_cnt += 1,
                None => break,
                _ => continue,
            }
        }
        self.pos = (0, 0);
        tree_cnt
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let input = buffer.trim_end().split("\n").collect();
    let mut grid = Grid::parse(input).ok_or(new_err("Failed parsing"))?;

    println!("Trees: {}", grid.loop_grid(3, 1));
    let aboreal_stop: u32 = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|(r, d)| grid.loop_grid(*r, *d))
        .product();
    println!("Aboreal stop prob: {}", aboreal_stop);
    Ok(())
}
