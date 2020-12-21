use regex::Regex;
use std::collections::{HashMap, HashSet};

use std::error::Error;
use std::io::{self, Read};

#[derive(Debug)]
struct AlergyProc {
    may_contain: HashMap<String, Vec<Vec<String>>>,
    all_ing: HashSet<String>,
    raw_data: Vec<HashSet<String>>,
}

impl AlergyProc {
    fn parse_foods(foods: &Vec<&str>) -> AlergyProc {
        let mut may_contain: HashMap<String, Vec<Vec<String>>> = HashMap::new();
        let mut all_ing = HashSet::new();
        let mut raw_data = Vec::new();

        let food_re = Regex::new(r"((?:[a-z]+ )+)\(contains (.*)\)").unwrap();
        let word_re = Regex::new(r"([a-z]+)").unwrap();

        for line in foods.iter() {
            //println!("{:?}", line);
            if let Some(food_caps) = food_re.captures(line) {
                //println!("{:?}", food_caps);
                let ings = word_re
                    .captures_iter(&food_caps[1])
                    .map(|cap| cap[1].to_string())
                    .collect::<Vec<String>>();
                let alergs = word_re.captures_iter(&food_caps[2]);
                all_ing = all_ing
                    .union(&ings.iter().cloned().collect())
                    .cloned()
                    .collect();
                for alerg in alergs {
                    //println!("{:?}", alerg);
                    may_contain
                        .entry(alerg[1].to_string())
                        .or_insert(Vec::new())
                        .push(ings.iter().cloned().collect());
                }
                raw_data.push(ings.iter().cloned().collect());
            }
        }
        AlergyProc {
            may_contain,
            all_ing,
            raw_data,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let foods = buffer.trim_end().split("\n").collect();
    let proc = AlergyProc::parse_foods(&foods);
    let raw_data: Vec<HashSet<String>> = proc.raw_data.iter().cloned().collect();
    let mut candidates = proc
        .may_contain
        .into_iter()
        .map(|(alerg, ing_lst)| {
            let mut ing_iter = ing_lst.iter();
            if let Some(shortest) = ing_iter.next() {
                let a = shortest.iter().cloned().collect::<HashSet<String>>();
                (
                    alerg,
                    ing_lst.iter().fold(a, |acc, ings| {
                        acc.intersection(&ings.iter().cloned().collect())
                            .cloned()
                            .collect()
                    }),
                )
            } else {
                (alerg, HashSet::new())
            }
        })
        .collect::<HashMap<String, _>>();
    println!("Candidates: {:?}", candidates);
    let alergy_ing = candidates.values().flatten().cloned().collect();
    println!("Alergy ing: {:?}", alergy_ing);
    let alergy_free: HashSet<String> = proc.all_ing.difference(&alergy_ing).cloned().collect();
    println!("Alergy free: {:?}", alergy_free);
    let res = alergy_free
        .iter()
        .map(|ing| raw_data.iter().filter(|f| f.contains(ing)).count())
        .sum::<usize>();
    println!("Res: {:?}", res);
    let mut found: HashMap<String, String> = HashMap::new();
    while !candidates.is_empty() {
        let (a, ing) = candidates
            .iter()
            .filter(|(_, i)| i.len() == 1)
            .next()
            .unwrap();
        let a = a.to_string();

        let known_ing = ing.into_iter().next().unwrap().to_string();
        candidates
            .iter_mut()
            .map(|(_, i)| i.remove(&known_ing))
            .for_each(drop);
        candidates.remove(&a);
        found.insert(a, known_ing);
    }
    let mut danger_list = found.iter().collect::<Vec<_>>();
    danger_list.sort_by_key(|(a, _)| *a);
    let res = danger_list
        .iter()
        .map(|(_, i)| *i)
        .cloned()
        .collect::<Vec<String>>()
        .join(",");
    println!("Res: {:?}", res);

    Ok(())
}
