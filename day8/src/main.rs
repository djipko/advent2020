use regex::Regex;
use std::collections::HashSet;
use std::error::Error;
use std::io::{self, Read};

#[derive(Debug, Clone)]
enum Ins {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

#[derive(Debug)]
enum State {
    Running(i32),
    Term(i32),
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
            .map(|line| match ins_re.captures(line) {
                Some(caps) => match &caps[1] {
                    "acc" => Some(Ins::Acc(caps[2].to_string().parse().unwrap())),
                    "jmp" => Some(Ins::Jmp(caps[2].to_string().parse().unwrap())),
                    "nop" => Some(Ins::Nop(caps[2].to_string().parse().unwrap())),
                    _ => None,
                },
                None => None,
            })
            .collect::<Option<Vec<Ins>>>()?;
        Some(GameConsole {
            acc: 0,
            program,
            pc: 0,
        })
    }

    fn step(&mut self) -> State {
        match self.program.get(self.pc as usize) {
            Some(ins) => match ins {
                Ins::Acc(val) => {
                    self.acc += val;
                    self.pc += 1;
                }
                Ins::Jmp(val) => self.pc += val,
                Ins::Nop(_) => self.pc += 1,
            },
            None => return State::Term(self.acc),
        }
        State::Running(self.acc)
    }

    fn run(&mut self) -> State {
        let mut seen = HashSet::new();
        loop {
            let state = self.step();
            match state {
                State::Running(_) => {
                    if seen.contains(&self.pc) {
                        break state;
                    }
                    seen.insert(self.pc);
                }
                State::Term(_) => break state,
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let mut console =
        GameConsole::load(buffer.trim_end().split("\n").collect()).ok_or("Failed parsing")?;
    println!("Acc is at {:?}", console.run());
    for (i, op) in console.program.iter().enumerate() {
        let patched = match op {
            Ins::Jmp(val) => {
                let mut patched = console.program.clone();
                patched[i] = Ins::Nop(*val);
                patched
            }
            Ins::Nop(val) => {
                let mut patched = console.program.clone();
                patched[i] = Ins::Jmp(*val);
                patched
            }
            _ => continue,
        };
        let mut new = GameConsole {
            acc: 0,
            program: patched,
            pc: 0,
        };
        if let State::Term(acc) = new.run() {
            println!("Patched: {}", acc);
            break;
        };
    }

    Ok(())
}
