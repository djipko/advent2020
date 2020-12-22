use std::collections::VecDeque;
use std::error::Error;
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let input_decks = buffer.trim_end().split("\n\n");
    let mut decks: Vec<VecDeque<usize>> = input_decks
        .map(|i| {
            i.split("\n")
                .skip(1)
                .map(str::parse)
                .collect::<Result<VecDeque<usize>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?;

    while decks.iter().all(|d| !d.is_empty()) {
        let mut drawn = decks
            .iter_mut()
            .map(|d| d.pop_front().unwrap())
            .enumerate()
            .collect::<Vec<_>>();
        drawn.sort_by_key(|(_, c)| *c);
        let (won, _) = drawn.iter().rev().nth(0).unwrap();
        drawn
            .iter()
            .rev()
            .for_each(|(_, c)| decks.get_mut(*won).unwrap().push_back(*c));
    }
    let scores: Vec<usize> = decks
        .iter()
        .filter(|d| !d.is_empty())
        .map(|d| {
            d.iter()
                .rev()
                .enumerate()
                .map(|(i, c)| (i + 1) * c)
                .sum::<usize>()
        })
        .collect();

    println!("Scores: {:?}", scores);

    Ok(())
}
