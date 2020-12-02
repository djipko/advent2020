use regex::Regex;
use std::error::Error;
use std::io::{self, Read};
use string_error::*;

#[derive(Debug)]
struct PasswordRule {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

impl PasswordRule {
    fn parse(rule: &str) -> Option<PasswordRule> {
        let re = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").ok()?;
        let captures = re.captures(rule)?;
        let min = captures.get(1)?.as_str().parse().ok()?;
        let max = captures.get(2)?.as_str().parse().ok()?;
        let letter = captures.get(3)?.as_str().chars().next()?;
        let password = captures.get(4)?.as_str();
        Some(PasswordRule {
            min,
            max,
            letter,
            password: String::from(password),
        })
    }
    fn check(&self) -> bool {
        let occ = self.password.chars().filter(|c| *c == self.letter).count();
        if occ < self.min || occ > self.max {
            false
        } else {
            true
        }
    }

    fn check2(&self) -> bool {
        let fst = self.password.chars().nth(self.min - 1).unwrap() == self.letter;
        let snd = self.password.chars().nth(self.max - 1).unwrap() == self.letter;
        fst ^ snd
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let passwords = buffer
        .trim_end()
        .split("\n")
        .map(|rule| PasswordRule::parse(&rule))
        .collect::<Option<Vec<PasswordRule>>>()
        .ok_or(new_err("Failed parsing"))?;
    println!("{:?}", passwords);
    println!(
        "Valid: {}",
        passwords.iter().filter(|pw| pw.check()).count()
    );
    println!(
        "Valid2: {}",
        passwords.iter().filter(|pw| pw.check2()).count()
    );
    Ok(())
}
