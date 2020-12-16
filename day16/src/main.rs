use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read};
use std::ops::RangeInclusive;

type Ticket = Vec<usize>;

#[derive(Debug)]
struct Rules {
    rules: HashMap<String, Vec<RangeInclusive<usize>>>,
}

impl Rules {
    fn parse(input: Vec<&str>) -> Option<Rules> {
        let rule_re = Regex::new(r"([a-z ]+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
        let data = input
            .iter()
            .map(|line| {
                println!("{}", line);
                if let Some(caps) = rule_re.captures(line) {
                    println!("{:?}", caps);
                    let fst_range = caps[2].parse().ok()?..=caps[3].parse().ok()?;
                    let snd_range = caps[4].parse().ok()?..=caps[5].parse().ok()?;
                    Some((caps[1].to_string(), vec![fst_range, snd_range]))
                } else {
                    None
                }
            })
            .collect::<Option<Vec<_>>>()?;

        Some(Rules {
            rules: data.into_iter().collect(),
        })
    }

    fn find_invalid(&self, ticket: &Ticket) -> Option<usize> {
        let mut ret = None;
        for n in ticket.iter() {
            if !self.rules.values().flatten().any(|r| r.contains(n)) {
                println!("Invalid: {:?}, {}", ticket, n);
                ret = Some(*n);
                break;
            }
        }
        ret
    }
}

fn parse_ticket(input: &str) -> Option<Ticket> {
    input
        .split(",")
        .map(|n| n.parse().ok())
        .collect::<Option<Ticket>>()
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut input_parts = buffer.trim_end().split("\n\n");
    let rule_str = input_parts.next().ok_or("Rules not found")?;
    let my_ticket_str = input_parts.next().ok_or("My ticket not found")?;
    let tickets_str = input_parts.next().ok_or("Tickets not found")?;

    let rules = Rules::parse(rule_str.split("\n").collect()).ok_or("Failed parsing")?;
    let tickets = tickets_str
        .split("\n")
        .skip(1)
        .map(|ts| parse_ticket(ts))
        .collect::<Option<Vec<Ticket>>>()
        .ok_or("Failed parsing tix")?;
    println!("Err rate: {}",
        tickets.iter().map(|t| rules.find_invalid(t).unwrap_or(0)).sum::<usize>()
    );

    Ok(())
}
