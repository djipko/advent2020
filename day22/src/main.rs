use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::io::{self, Read};

fn play(decks: &mut Vec<VecDeque<usize>>) {
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
}

fn play_recc(decks: &mut Vec<VecDeque<usize>>) {
    let mut seen = HashSet::new();
    while decks.iter().all(|d| !d.is_empty()) {
        //println!("{:?}", decks);
        let decks_key = format!("{:?}", decks);
        if seen.contains(&decks_key) {
            //println!("Found: {:?}", decks);
            decks.iter_mut().skip(1).for_each(|d| d.clear());
            return
        } else {
            seen.insert(decks_key);
        };
        let mut drawn = decks
            .iter_mut()
            .map(|d| d.pop_front().unwrap())
            .enumerate()
            .collect::<Vec<_>>();
        if drawn.iter().all(|(di, c)| decks[*di].len() >= *c) {
            let mut decks_copy = decks
                .iter()
                .zip(drawn.iter())
                .map(|(d, (_, c))| d.iter().cloned().take(*c).collect())
                .collect();
            play_recc(&mut decks_copy);
            let (i, _) = decks_copy
                .iter()
                .enumerate()
                .filter(|(_, d)| !d.is_empty())
                .nth(0)
                .unwrap();
            drawn.sort_by_key(|(id, _)| *id == i);
            drawn
                .iter()
                .rev()
                .for_each(|(_, c)| decks.get_mut(i).unwrap().push_back(*c));
        } else {
            drawn.sort_by_key(|(_, c)| *c);
            let (won, _) = drawn.iter().rev().nth(0).unwrap();
            drawn
                .iter()
                .rev()
                .for_each(|(_, c)| decks.get_mut(*won).unwrap().push_back(*c));
        }
    }
}

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
    let mut decks2 = decks.iter().cloned().collect();

    play(&mut decks);
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
    play_recc(&mut decks2);
    let scores2: Vec<usize> = decks2
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
    println!("Final: {:?}", decks2);
    println!("Scores: {:?}", scores2);

    Ok(())
}
