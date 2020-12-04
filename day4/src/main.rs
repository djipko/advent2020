use regex::Regex;
use std::error::Error;
use std::io::{self, Read};

#[derive(Debug)]
enum Field {
    byr(String),
    iyr(String),
    eyr(String),
    hgt(String),
    hcl(String),
    ecl(String),
    pid(String),
    cid(String),
}

impl Field {
    fn is_valid(&self) -> bool {
        match self {
            Field::byr(data) => data
                .parse::<u32>()
                .map_or(false, |year| year >= 1920 && year <= 2002),
            Field::iyr(data) => data
                .parse::<u32>()
                .map_or(false, |year| year >= 2010 && year <= 2020),
            Field::eyr(data) => data
                .parse::<u32>()
                .map_or(false, |year| year >= 2020 && year <= 2030),
            Field::hgt(data) => {
                let re = Regex::new(r"(\d+)(in|cm)").unwrap();
                if let Some(caps) = re.captures(data) {
                    if let Some(unit) = caps.get(2) {
                        match unit.as_str() {
                            "cm" => caps.get(1).map_or(false, |hs| {
                                hs.as_str()
                                    .parse::<u32>()
                                    .map_or(false, |h| h >= 150 && h <= 193)
                            }),
                            "in" => caps.get(1).map_or(false, |hs| {
                                hs.as_str()
                                    .parse::<u32>()
                                    .map_or(false, |h| h >= 59 && h <= 76)
                            }),
                            _ => false,
                        }
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            Field::hcl(data) => {
                let re = Regex::new(r"#[a-f0-9]{6}$").unwrap();
                re.is_match(data)
            }
            Field::ecl(data) => {
                let re = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
                re.is_match(data)
            }
            Field::pid(data) => {
                let re = Regex::new(r"^[0-9]{9}$").unwrap();
                re.is_match(data)
            }
            _ => true,
        }
    }
}

#[derive(Debug)]
struct Passport {
    fields: Vec<Field>,
}

impl Passport {
    fn parse_all(input: Vec<String>) -> Option<Vec<Passport>> {
        input
            .iter()
            .map(|l| Passport::parse(l))
            .collect::<Option<Vec<Passport>>>()
    }

    fn parse(line: &String) -> Option<Passport> {
        let re = Regex::new(r"(byr|iyr|eyr|hgt|hcl|ecl|pid|cid):([#a-z0-9]+)").ok()?;
        let fields = re
            .captures_iter(&line)
            .map(|cap| match &cap[1] {
                "byr" => Some(Field::byr(cap[2].to_string())),
                "iyr" => Some(Field::iyr(cap[2].to_string())),
                "eyr" => Some(Field::eyr(cap[2].to_string())),
                "hgt" => Some(Field::hgt(cap[2].to_string())),
                "hcl" => Some(Field::hcl(cap[2].to_string())),
                "ecl" => Some(Field::ecl(cap[2].to_string())),
                "pid" => Some(Field::pid(cap[2].to_string())),
                "cid" => Some(Field::cid(cap[2].to_string())),
                _ => None,
            })
            .collect();
        if let Some(fields) = fields {
            Some(Passport { fields })
        } else {
            None
        }
    }

    fn is_valid(&self) -> bool {
        let cit = self
            .fields
            .iter()
            .filter(|f| if let Field::cid(_) = f { true } else { false })
            .count()
            != 0;
        (self.fields.len() == 7 && !cit) || self.fields.len() == 8
    }

    fn is_valid2(&self) -> bool {
        self.is_valid() && self.fields.iter().all(|fld| fld.is_valid())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let input = buffer
        .split("\n\n")
        .map(|l| str::replace(l, "\n", " "))
        .collect::<Vec<String>>();
    let passports = Passport::parse_all(input).ok_or("Failed parsing")?;
    println!(
        "Valid: {}",
        passports.iter().filter(|p| p.is_valid()).count()
    );
    println!(
        "Valid2: {}",
        passports.iter().filter(|p| p.is_valid2()).count()
    );
    Ok(())
}
