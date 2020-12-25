use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::{self, Read};

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
        Tile { color: Color::W }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Field(i32, i32);

#[derive(Debug)]
struct Floor {
    tiles: HashMap<(i32, i32), Tile>,
    conway: HashSet<Field>,
}

impl Floor {
    fn new() -> Floor {
        Floor {
            tiles: HashMap::new(),
            conway: HashSet::new(),
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

    fn make_conway(&mut self) {
        self.conway = self
            .tiles
            .iter()
            .filter(|(_, t)| t.color == Color::B)
            .map(|(c, _)| Field(c.0, c.1))
            .collect();
    }

    fn conway_step(&mut self) {
        let inactive_n = self
            .conway
            .iter()
            .map(|fld| {
                fld.neighbours()
                    .difference(&self.conway)
                    .cloned()
                    .collect::<HashSet<Field>>()
            })
            .fold(HashSet::new(), |acc, x| acc.union(&x).cloned().collect());
        //println!("inactive_n: {:?}", inactive_n);
        let new_active: HashSet<Field> = inactive_n
            .into_iter()
            .filter(|f| f.neighbours().intersection(&self.conway).count() == 2)
            .collect();
        //println!("new_active: {:?}", new_active);
        let still_active = self
            .conway
            .iter()
            .filter(|f| {
                let active_n = f.neighbours().intersection(&self.conway).count();
                active_n != 0 && active_n <= 2
            })
            .cloned()
            .collect();
        //println!("still_active: {:?}", still_active);
        self.conway = new_active.union(&still_active).cloned().collect();
    }
}

impl Field {
    fn neighbours(&self) -> HashSet<Field> {
        vec![(2, 0), (1, -1), (-1, -1), (-2, 0), (-1, 1), (1, 1)]
            .iter()
            .map(|(x, y)| Field(self.0 + x, self.1 + y))
            .collect()
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
    let runs = lines
        .map(|line| {
            let mut res = Vec::new();
            println!("Checking: {}", line);
            parse_run(line, &d, &mut res);
            res
        })
        .collect::<Vec<_>>();
    let mut floor = Floor::new();
    for run in runs {
        //println!("Doing run: {:?}", run);
        floor.do_run(&run);
        //println!("Floor: {:?}", floor);
    }
    println!(
        "{}",
        floor
            .tiles
            .iter()
            .filter(|(_, tile)| tile.color == Color::B)
            .count()
    );
    floor.make_conway();
    for i in 0..100 {
        println!("step {}", i);
        floor.conway_step();
    }
    println!("{}", floor.conway.len());

    Ok(())
}
