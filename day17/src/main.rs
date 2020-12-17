use itertools::{iproduct, Itertools};
use std::collections::HashSet;
use std::error::Error;
use std::io::{self, Read};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Field(i32, i32, i32, i32);

#[derive(Debug)]
struct Grid {
    fields: HashSet<Field>,
}

impl Grid {
    fn parse(input: Vec<&str>) -> Grid {
        let fields = input
            .iter()
            .enumerate()
            .map(|(row, cells)| {
                cells
                    .chars()
                    .enumerate()
                    .filter(|(_, cell)| *cell == '#')
                    .map(move |(col, _)| Field(col as i32, row as i32, 0, 0))
            })
            .flatten()
            .collect::<HashSet<Field>>();
        Grid { fields }
    }

    fn step(&mut self) {
        let inactive_n = self
            .fields
            .iter()
            .map(|fld| {
                fld.neighbours()
                    .difference(&self.fields)
                    .cloned()
                    .collect::<HashSet<Field>>()
            })
            .fold(HashSet::new(), |acc, x| acc.union(&x).cloned().collect());
        //println!("inactive_n: {:?}", inactive_n);
        let new_active: HashSet<Field> = inactive_n
            .iter()
            .filter(|f| f.neighbours().intersection(&self.fields).count() == 3)
            .cloned()
            .collect();
        //println!("new_active: {:?}", new_active);
        let still_active = self
            .fields
            .iter()
            .filter(|f| {
                let active_n = f.neighbours().intersection(&self.fields).count();
                active_n == 2 || active_n == 3
            })
            .cloned()
            .collect();
        //println!("still_active: {:?}", still_active);
        self.fields = new_active.union(&still_active).cloned().collect();
    }
}

impl Field {
    fn neighbours(&self) -> HashSet<Field> {
        iproduct!(
            vec![self.0 - 1, self.0, self.0 + 1],
            vec![self.1 - 1, self.1, self.1 + 1],
            vec![self.2 - 1, self.2, self.2 + 1],
            vec![self.3 - 1, self.3, self.3 + 1]
        )
        .map(|(x, y, z, w)| Field(x, y, z, w))
        .filter(|f| f != self)
        .collect()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let input = buffer.trim_end().split("\n").collect();
    let mut grid = Grid::parse(input);
    //println!("Grid: {:?}", grid);
    for _ in 0..6 {
        grid.step();
        //println!("Grid: {:?}", grid);
    }
    println!("Active: {:?}", grid.fields.len());
    Ok(())
}
