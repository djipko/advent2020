use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::io::{self, Read};
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
enum Color {
    B,
    W,
}

#[derive(Debug)]
struct Tile {
    color: Color,
}

impl Tile {
    fn new() -> Tile {
        Tile {
            color: Color::W,
        }
    }
}

#[derive(Debug)]
struct Floor {
    tiles: HashMap<(i32, i32), Tile>,
}

impl Floor {
    fn new() -> Floor {
        let tiles = HashMap::new();
        Floor {
            tiles,
        }
    }

    fn do_run(&mut self, run: &Vec<String>) {
        let mut x = 0;
        let mut y = 0;
        for d in run.iter() {
            let m = match &d[..] {
                "e" => (2, 0),
                "se" => (1, -1),
                "sw" => (-1, -1),
                "w" => (-2, 0),
                "nw" => (-1, 1),
                "ne" => (1, 1),
                _ => (0, 0),
            };
            x += m.0;
            y += m.1;
        }
        let mut tile = self.tiles.entry((x, y)).or_insert(Tile::new());
        let new_col = match tile.color {
            Color::W => Color::B,
            Color::B => Color::W,
        };
        tile.color = new_col;
    }
}

fn parse_run(input: &str, dict: &HashSet<&str>, found: &mut Vec<String>) {
    //println!("Checking: {}", input);
    if input.len() < 2 {
        found.push(input.to_string());
        return;
    }
    for i in 1..=2 {
        //println!("slice: {}", &input[..i]);
        if dict.contains(&input[..i]) {
            let mut next = Vec::new();
            parse_run(&input[i..], dict, &mut next);
            if next.len() > 0 {
                found.push(input[..i].to_string());
                found.extend(next);
                return;
            }
        }
    }
    found.clear();
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let lines = buffer.trim_end().split("\n");
    let d = vec!["e", "se", "sw", "w", "nw", "ne"].into_iter().collect();
    let runs = lines.map(|line| {
        let mut res = Vec::new();
        println!("Checking: {}", line);
        parse_run(line, &d, &mut res);
        res
    }).collect::<Vec<_>>();
    let mut floor = Floor::new();
    for run in runs {
        println!("Doing run: {:?}", run);
        floor.do_run(&run);
        println!("Floor: {:?}", floor);
    };
    println!("{}", floor.tiles.iter().filter(|(_, tile)| tile.color == Color::B).count());

    Ok(())
}
