use std::collections::HashSet;
use std::error::Error;
use std::io::{self, Read};

fn group_answers(answers: &str) -> usize {
    let uniq: HashSet<char> = answers.chars().collect();
    uniq.len()
}

fn group_answers2(answers: &str) -> usize {
    let mut person_answers = answers.split("\n");
    if let Some(fst) = person_answers.next() {
        let mut common: HashSet<char> = fst.chars().collect();
        for ans in person_answers {
            common = common
                .intersection(&ans.chars().collect())
                .copied()
                .collect();
        }
        common.len()
    } else {
        0
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let input = buffer.split("\n\n").collect::<Vec<&str>>();

    println!(
        "Sum of yes answers {}",
        input
            .iter()
            .map(|group| { group_answers(&str::replace(group, "\n", "")) })
            .sum::<usize>()
    );
    println!(
        "Sum of common yes answers {}",
        input
            .iter()
            .map(|group| group_answers2(group))
            .sum::<usize>()
    );
    Ok(())
}
