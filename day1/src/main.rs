use std::error::Error;
use regex::Regex;
use std::io::{self, Read};

struct PasswordRule {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

impl PasswordRule {
    fn parse(rule: &String) -> Option<PasswordRule> {
        let re = Regex::new(r"(\d+)-(\d+) ([a..z]): ([a..z]+)")?;
        let captures = re.captures(rule)?;
        let min: usize = captures.get(1)?.as_str().parse()?;
        let max: usize = captures.get(2)?.as_str().parse()?;
        let letter: char = captures.get(3)?.as_str().chars().next()?;
        let password = captures.get(4)?.as_str();
        Some(PasswordRule {
            min, max, letter, password})
    }
    fn check(&self) -> bool {
        let occ = self.password.chars().filter(|c| *c == self.letter).count();
        if occ < self.min || occ > self.max {
            true
        } else {
            false
        }
    }

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let passwords = buffer
        .trim_end()
        .split("\n")
        .map(|rule| PasswordRule::parse(&rule))
        .collect::<Option<Vec<PasswordRule>>>().ok_or(Error::new("Failed parsing"))?;

}
