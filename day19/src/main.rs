use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read};

type RuleList = Vec<Vec<usize>>;

#[derive(Debug)]
enum Rule {
    Char(String),
    RuleList(RuleList),
}

#[derive(Debug)]
struct Rules {
    rules: HashMap<usize, Rule>,
}

impl Rules {
    fn parse_rulelist(rules: &str) -> Option<Vec<usize>> {
        let data_re = Regex::new(r"(\d+)").unwrap();
        data_re
            .captures_iter(rules)
            .map(|data_caps| {
                //println!("capslist {:?}", data_caps);
                Some(data_caps[1].parse().ok()?)
            })
            .collect::<Option<Vec<usize>>>()
    }

    fn parse_rules(input: Vec<&str>) -> Option<Rules> {
        let rule_re = Regex::new(r"(\d+): (.*)").unwrap();
        let char_re = Regex::new(r#""([a-z])""#).unwrap();
        let or_re = Regex::new(r"(.*) \| (.*)").unwrap();
        let rules = input
            .iter()
            .map(|line| {
                println!("{}", line);
                if let Some(id_caps) = rule_re.captures(line) {
                    //println!("{:?}", id_caps);
                    let id = id_caps[1].parse::<usize>().ok()?;
                    if let Some(char_caps) = char_re.captures(&id_caps[2]) {
                        return Some((id, Rule::Char(char_caps[1].to_string())));
                    }
                    if let Some(or_caps) = or_re.captures(&id_caps[2]) {
                        //println!("{:?}", or_caps);
                        Some((
                            id,
                            Rule::RuleList(vec![
                                Self::parse_rulelist(&or_caps[1])?,
                                Self::parse_rulelist(&or_caps[2])?,
                            ]),
                        ))
                    } else {
                        Some((id, Rule::RuleList(vec![Self::parse_rulelist(&id_caps[2])?])))
                    }
                } else {
                    None
                }
            })
            .collect::<Option<HashMap<usize, Rule>>>()?;
        Some(Rules { rules })
    }

    fn matches_rule_rec<'a>(&self, msg: &'a str, rule_id: usize) -> (bool, &'a str) {
        if let Some(rule) = self.rules.get(&rule_id) {
            println!("Matching {}: {:?} to {}", rule_id, rule, msg);
            match rule {
                Rule::Char(s) => {
                    let ret = (s == &msg[0..1], &msg[1..]);
                    println!("Returning {} -> {:?}", rule_id, ret);
                    ret
                }
                Rule::RuleList(options) => {
                    let matched = options
                        .iter()
                        .map(|option| {
                            let mut rest = msg;
                            let mut mch = true;
                            for rid in option {
                                let ret = self.matches_rule_rec(rest, *rid);
                                mch = ret.0;
                                rest = ret.1;

                                if !mch {
                                    break;
                                }
                            }
                            (mch, rest)
                        })
                        .filter(|(m, _)| *m)
                        .nth(0);
                    let ret = if let Some((m, rest)) = matched {
                        (m, rest)
                    } else {
                        (false, "")
                    };

                    println!("Returning {} -> {:?}", rule_id, ret);
                    ret
                }
            }
        } else {
            (false, "")
        }
    }

    fn matches_rule(&self, msg: &str, rule_id: usize) -> bool {
        let ret = self.matches_rule_rec(msg, rule_id);
        ret.0 && ret.1.len() == 0
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut input_parts = buffer.trim_end().split("\n\n");
    let rule_str = input_parts.next().ok_or("Rules not found")?;
    let msg_string = input_parts.next().ok_or("Messages not found")?;
    let rules = Rules::parse_rules(rule_str.split("\n").collect()).ok_or("Failed parsing rules")?;
    println!("Rules: {:?}", rules);
    let c = msg_string
        .split("\n")
        .filter(|msg| {
            println!("Trying msg: {}", msg);
            rules.matches_rule(msg, 0)
        })
        .count();
    println!("Res: {}", c);

    Ok(())
}
