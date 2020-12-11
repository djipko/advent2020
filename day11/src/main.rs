use itertools::{iproduct, Itertools};
use std::error::Error;
use std::io::{self, Read};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Field {
    Empty,
    Occ,
    Floor,
}

#[derive(Debug)]
struct Grid {
    map: Vec<Vec<Field>>,
    w: i32,
    h: i32,
}

impl Grid {
    fn parse(input: Vec<&str>) -> Option<Grid> {
        let h = input.len() as i32;
        let map = input
            .iter()
            .map(|row| {
                row.chars()
                    .map(|cell| match cell {
                        'L' => Some(Field::Empty),
                        '#' => Some(Field::Occ),
                        '.' => Some(Field::Floor),
                        _ => None,
                    })
                    .collect::<Option<Vec<Field>>>()
            })
            .collect::<Option<Vec<Vec<Field>>>>()?;
        let w = map.first()?.len() as i32;
        Some(Grid { map, w, h })
    }

    fn neighbours(&self, x: i32, y: i32) -> Vec<(i32, i32)> {
        iproduct!(vec![x - 1, x, x + 1], vec![y - 1, y, y + 1])
            .filter(|(cx, cy)| {
                (cx, cy) != (&x, &y) && cx >= &0 && cx < &self.w && cy >= &0 && cy < &self.h
            })
            .collect()
    }

    fn should_fill(&self, x: i32, y: i32) -> bool {
        self.neighbours(x, y)
            .iter()
            .all(|(xx, yy)| match self.map[*yy as usize][*xx as usize] {
                Field::Empty | Field::Floor => true,
                _ => false,
            })
    }

    fn should_leave(&self, x: i32, y: i32) -> bool {
        self.neighbours(x, y)
            .iter()
            .filter(|(xx, yy)| match self.map[*yy as usize][*xx as usize] {
                Field::Occ => true,
                _ => false,
            })
            .count()
            >= 4
    }

    fn step(&mut self) -> i32 {
        let mut changed = 0;
        let new_map = self
            .map
            .iter()
            .cloned()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate().map(|(x, fld)| {
                        //println!("{}, {}, {:?} {:?}", x, y, fld, self.neighbours(x as i32, y as i32));
                        match fld {
                        Field::Empty => {
                            if self.should_fill(x as i32, y as i32) {
                                changed += 1;
                                Field::Occ
                            } else {
                                Field::Empty
                            }
                        }
                        Field::Occ => {
                            if self.should_leave(x as i32, y as i32) {
                                changed += 1;
                                Field::Empty
                            } else {
                                Field::Occ
                            }
                        }
                        _ => *fld,
                    }})
                    .collect()
            })
            .collect();
        self.map = new_map;
        changed
    }

    fn total_occ(&self) -> i32 {
        self.map.iter().map(|row| row.iter().filter(|f| **f == Field::Occ).count() as i32).sum()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let input = buffer.trim_end().split("\n").collect();
    let mut grid = Grid::parse(input).ok_or("Failed parsing")?;
    loop {
        if grid.step() == 0 {
            break
        }
    };
    println!("Occupied: {}", grid.total_occ());
    Ok(())
}
