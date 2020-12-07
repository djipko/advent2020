use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::io::{self, Read};

#[derive(Debug)]
struct BagRule {
    color: String,
    count: usize,
}

#[derive(Debug)]
struct BagRules {
    rules: HashMap<String, Vec<BagRule>>,
    all_contained: HashSet<String>,
}

impl BagRules {
    fn parse_rules(input: Vec<&str>) -> Option<BagRules> {
        let mut rules = HashMap::new();
        let mut all_contained = HashSet::new();
        let rule_re = Regex::new(
            r"([a-z]+ [a-z]+) bags contain ((?:\d (?:[a-z]+ [a-z]+) bag[s]?(?:, )?)+|(?:no other bags))\.").unwrap();
        let cont_re = Regex::new(r"((\d) ([a-z]+ [a-z]+)) bag[s]?").unwrap();
        for line in input.iter() {
            //println!("{}", line);
            //println!("{:#?}", rule_re.captures(line));
            if let Some(caps) = rule_re.captures(line) {
                let containing = String::from(&caps[1]);
                if let None = rules.get(&containing) {
                    rules.insert(containing.clone(), Vec::new());
                }
                let contained = &caps[2];
                for cap in cont_re.captures_iter(contained) {
                    //println!("{}: {:?}", containing, cap);
                    let color = cap[3].to_string();
                    let rule = BagRule {
                        color: color.clone(),
                        count: cap[2].to_string().parse().unwrap(),
                    };
                    all_contained.insert(color);
                    rules.get_mut(&containing)?.push(rule);
                }
            }
        }
        Some(BagRules {
            rules,
            all_contained,
        })
    }

    fn count_bags(&self, needle: &str) -> usize {
        let all_containing = self.rules.keys();
        all_containing
            .filter(|start| self.contains(start, needle))
            .count()
    }

    fn contains(&self, from: &str, to: &str) -> bool {
        match self.rules.get(from) {
            Some(contained) => {
                if contained.iter().filter(|b| b.color == to).count() > 0 {
                    true
                } else {
                    contained.iter().any(|b| self.contains(&b.color, to))
                }
            }
            None => false,
        }
    }

    fn count_containing(&self, from: &str) -> usize {
        match self.rules.get(from) {
            Some(contained) => contained
                .iter()
                .map(|b| b.count + b.count * self.count_containing(&b.color))
                .sum(),
            None => 0,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let input = buffer.split("\n").collect();
    let rules = BagRules::parse_rules(input).ok_or("Failed parsing")?;
    //println!("{:#?}", rules);
    println!("Ways to hold: {}", rules.count_bags("shiny gold"));
    println!("Contained in: {}", rules.count_containing("shiny gold"));
    Ok(())
}
