use std::collections::HashMap;
use std::error::Error;

#[derive(Debug)]
struct Game {
    last_spoken: HashMap<usize, Vec<usize>>,
    turn: usize,
    last: usize,
    init: Vec<usize>,
}

impl Game {
    fn new(initial: Vec<usize>) -> Game {
        let mut last_spoken = HashMap::new();
        initial
            .iter()
            .enumerate()
            .for_each(|(t, n)| drop(last_spoken.insert(*n, vec![t + 1])));
        Game {
            last_spoken,
            turn: initial.len(),
            last: *initial.iter().last().unwrap(),
            init: initial.into_iter().rev().collect(),
        }
    }
}

impl Iterator for Game {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.init.is_empty() {
            return self.init.pop();
        }
        self.turn += 1;
        let new = if let Some(turns) = self.last_spoken.get(&self.last) {
            let mut turns_spoken = turns.iter().rev();
            let last_spoken = turns_spoken.next()?;
            let mut tmp = 0;
            if let Some(before_that) = turns_spoken.next() {
                tmp = last_spoken - before_that;
            }
            Some(tmp)
        } else {
            Some(0)
        }?;
        if let Some(v) = self.last_spoken.get_mut(&new) {
            v.push(self.turn);
        } else {
            self.last_spoken.insert(new, vec![self.turn]);
        }
        self.last = new;
        //println!("{:?}", self);
        Some(new)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let initial = vec![5, 2, 8, 16, 18, 0, 1];
    let mut game = Game::new(initial);
    println!("game: {:?}", game);
    println!("Res: {:?}", game.nth(2019));
    Ok(())
}
