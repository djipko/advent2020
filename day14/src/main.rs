use itertools::{iproduct, Itertools};
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read};

#[derive(Debug)]
enum Ins {
    Mask(String),
    Write(usize, u64),
}

#[derive(Debug)]
struct Program {
    mask0: u64,
    mask1: u64,
    mask: String,
    mem: HashMap<usize, u64>,
    addr_mode: bool,
}

impl Program {
    fn new(addr_mode: bool) -> Program {
        Program {
            mask0: u64::MAX,
            mask1: 0,
            mask: String::from("000000000000000000000000000000000000"),
            mem: HashMap::new(),
            addr_mode,
        }
    }

    fn parse_intruction(ins: &str) -> Option<Ins> {
        let ins_re = Regex::new(r"(?:mask = ([10X]{36})|mem\[(\d+)\] = (\d+))").unwrap();
        let caps = ins_re.captures(ins)?;
        match caps.get(1) {
            Some(mask) => Some(Ins::Mask(mask.as_str().to_string())),
            None => match (caps.get(2), caps.get(3)) {
                (Some(addr), Some(val)) => Some(Ins::Write(
                    addr.as_str().parse::<usize>().ok()?,
                    val.as_str().parse::<u64>().ok()?,
                )),
                _ => None,
            },
        }
    }

    fn exec_instruction(&mut self, ins: &Ins) {
        //println!("ins: {:?}", ins);
        match ins {
            Ins::Mask(mask) => {
                self.mask0 = u64::from_str_radix(&mask.replace("X", "1"), 2).unwrap();
                self.mask1 = u64::from_str_radix(&mask.replace("X", "0"), 2).unwrap();
                self.mask = mask.to_string();
            }
            Ins::Write(addr, val) => {
                if !self.addr_mode {
                    self.write_mode1(addr, val);
                } else {
                    self.write_mode2(addr, val);
                }
            }
        }
    }

    fn write_mode1(&mut self, addr: &usize, val: &u64) {
        let res = (val & self.mask0) | self.mask1;
        self.mem.insert(*addr, res);
    }

    fn write_mode2(&mut self, addr: &usize, val: &u64) {
        let addr_string = format!("{:036b}", addr);
        let addrs = addr_string
            .chars()
            .zip(self.mask.chars())
            .map(|(a, m)| if m == 'X' { vec!['0', '1'] } else { 
               if m == '1' { vec![m] } else {vec![a] }
            })
            .multi_cartesian_product()
            .map(|v| usize::from_str_radix(&v.into_iter().collect::<String>(), 2))
            .collect::<Result<Vec<usize>, _>>().unwrap();
        for addr in addrs.iter() {
            self.mem.insert(*addr, *val);
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let input: Vec<&str> = buffer.trim_end().split("\n").collect();
    let prog = input
        .into_iter()
        .map(Program::parse_intruction)
        .collect::<Option<Vec<Ins>>>()
        .ok_or("Failed parsing")?;
    let mut program = Program::new(false);
    prog.iter()
        .map(|ins| program.exec_instruction(ins))
        .for_each(drop);
    //println!("{:?}", prog);
    //println!("{:?}", program);
    println!("Res: {}", program.mem.values().sum::<u64>());
    let mut program = Program::new(true);
    prog.iter()
        .map(|ins| program.exec_instruction(ins))
        .for_each(drop);
    //println!("{:?}", prog);
    //println!("{:?}", program);
    println!("Res2: {}", program.mem.values().sum::<u64>());
    Ok(())
}
