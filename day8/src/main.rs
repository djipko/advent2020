use regex::Regex;
use std::collections::HashSet;
use std::error::Error;
use std::io::{self, Read};

#[derive(Debug)]
enum Ins {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

#[derive(Debug)]
struct GameConsole {
    acc: i32,
    program: Vec<Ins>,
    pc: i32,
}

impl GameConsole {
    fn load(input: Vec<&str>) -> Option<GameConsole> {
        let ins_re = Regex::new(r"(jmp|acc|nop) ([+-]\d+)").unwrap();
        let program = input
            .iter()
            .map(|line| {
                match ins_re.captures(line) {
                    Some(caps) => {
                        match &caps[1] {
                            "acc" => Some(Ins::Acc(caps[2].to_string().parse().unwrap())),
                            "jmp" => Some(Ins::Jmp(caps[2].to_string().parse().unwrap())),
                            "nop" => Some(Ins::Nop(caps[2].to_string().parse().unwrap())),
                            _ => None,
                        }
                    }
                    None => None,
                }
            })
            .collect::<Option<Vec<Ins>>>()?;
        Some(GameConsole {
            acc: 0,
            program,
            pc: 0,
        })
    }

    fn step(&mut self) {
        match self.program.get(self.pc as usize) {
            Some(ins) => match ins {
                Ins::Acc(val) => {
                    self.acc += val;
                    self.pc += 1;
                }
                Ins::Jmp(val) => self.pc += val,
                Ins::Nop(_) => self.pc += 1,
            },
            None => return,
        }
    }

    fn find_loop(&mut self) -> i32 {
        let mut seen = HashSet::new();
        loop {
            self.step();
            if seen.contains(&self.pc) {
                break self.acc;
            }
            seen.insert(self.pc);
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let mut console = GameConsole::load(buffer.trim_end().split("\n").collect()).ok_or("Failed parsing")?;
    println!("Acc is at {}", console.find_loop());
    Ok(())
}
