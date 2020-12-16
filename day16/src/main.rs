use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::{self, Read};
use std::ops::RangeInclusive;

type Ticket = Vec<usize>;

#[derive(Debug)]
struct Rules {
    rules: HashMap<String, Vec<RangeInclusive<usize>>>,
    field_mapping: HashMap<String, usize>,
}

struct Multizip<T>(Vec<T>);

impl<T> Iterator for Multizip<T>
where
    T: Iterator,
{
    type Item = Vec<T::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.iter_mut().map(Iterator::next).collect()
    }
}

impl Rules {
    fn parse(input: Vec<&str>) -> Option<Rules> {
        let rule_re = Regex::new(r"([a-z ]+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
        let data = input
            .iter()
            .map(|line| {
                if let Some(caps) = rule_re.captures(line) {
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
            field_mapping: HashMap::new(),
        })
    }

    fn find_invalid(&self, ticket: &Ticket) -> Option<usize> {
        let mut ret = None;
        for n in ticket.iter() {
            if !self.rules.values().flatten().any(|r| r.contains(n)) {
                ret = Some(*n);
                break;
            }
        }
        ret
    }

    fn build_field_mapping(&mut self, valid: &Vec<Ticket>) {
        let valid_it = valid.iter().map(|v| v.iter()).collect();
        let candidates = &mut HashMap::new();
        for (pos, codes) in Multizip(valid_it).enumerate() {
            let matched = self
                .rules
                .iter()
                .filter(|(_, rule)| {
                    codes
                        .iter()
                        .all(|code| rule.iter().any(|r| r.contains(code)))
                })
                .map(|(fld, _)| fld.to_string())
                .collect::<HashSet<String>>();
            candidates.insert(pos, matched);
        };
        while candidates.len() > 0 {
            let found_candidates: Vec<(usize, HashSet<String>)> = candidates
                .clone()
                .into_iter()
                .filter(|(_, flds)| flds.len() == 1)
                .collect();
            for (pos, flds) in found_candidates {
                let fld = flds.iter().next().unwrap().to_string();
                self.field_mapping.insert(fld.clone(), pos);
                candidates
                    .iter_mut()
                    .for_each(|(_, c)| drop(c.remove(&fld)));
                candidates.remove(&pos);
            }
        }
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

    let mut rules = Rules::parse(rule_str.split("\n").collect()).ok_or("Failed parsing")?;
    let tickets = tickets_str
        .split("\n")
        .skip(1)
        .map(|ts| parse_ticket(ts))
        .collect::<Option<Vec<Ticket>>>()
        .ok_or("Failed parsing tix")?;
    println!(
        "Err rate: {}",
        tickets
            .iter()
            .map(|t| rules.find_invalid(t).unwrap_or(0))
            .sum::<usize>()
    );
    let valid_tix = tickets
        .iter()
        .cloned()
        .filter(|t| rules.find_invalid(t) == None)
        .collect();
    rules.build_field_mapping(&valid_tix);
    let my_ticket = parse_ticket(my_ticket_str.split("\n").nth(1).unwrap()).ok_or("Failed parsing my ticket")?;
    let res: usize = rules.field_mapping.iter()
        .filter(|(fld, _)| fld.starts_with("departure"))
        .map(|(_, pos)| my_ticket[*pos]).product();
    println!("Res: {}", res);

    Ok(())
}
