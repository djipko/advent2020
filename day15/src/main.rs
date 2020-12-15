use std::collections::HashMap;
use std::collections::VecDeque;
use std::error::Error;

#[derive(Debug)]
struct Game {
    last_spoken: HashMap<usize, VecDeque<usize>>,
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
            .for_each(|(t, n)| drop(last_spoken.insert(*n, vec![t + 1].into_iter().collect())));
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
            let last_spoken = turns.back()?;
            let before_that = turns.front()?;
            Some(last_spoken - before_that)
        } else {
            Some(0)
        }?;
        let v = self.last_spoken.entry(new).or_insert(VecDeque::new());
        if v.len() == 2 {
            v.pop_front();
        };
        v.push_back(self.turn);
        self.last = new;
        //println!("{:?}", self);
        Some(new)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let initial = vec![5, 2, 8, 16, 18, 0, 1];
    let mut game = Game::new(initial.clone());
    println!("Res: {:?}", game.nth(2020 - 1));
    let mut game = Game::new(initial);
    println!("Res: {:?}", game.nth(30000000 - 1));
    Ok(())
}
