use std::collections::HashSet;
use std::error::Error;
use std::io::{self, Read};

fn group_answers(answers: &str) -> usize {
    let uniq: HashSet<char> = answers.chars().collect();
    uniq.len()
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let input = buffer
        .split("\n\n")
        .map(|l| str::replace(l, "\n", ""))
        .collect::<Vec<String>>();
    println!("Sum of yes answers {}", input.iter().map(|group| group_answers(&group)).sum::<usize>());
    Ok(())
}
