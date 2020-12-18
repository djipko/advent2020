use std::error::Error;
use std::io::{self, Read};

#[derive(Debug)]
enum Op {
    Add,
    Mul,
}

fn parse_op(ops: &str) -> Op {
    if ops == "+" {
        Op::Add
    } else {
        Op::Mul
    }
}

fn eval(input: &str, running: i64, run_op: Op) -> (i64, &str) {
    println!("{}: {}, {:?} ", input, running, run_op);
    let mut toks = input.splitn(3, " ");
    let lhs = toks.next().unwrap();
    if lhs.starts_with('(') {
        let (res, rest) = eval(&input[1..], 0, Op::Add);
        println!("{} -> {}", res, rest);
        let new_running = match run_op {
            Op::Add => running + res,
            Op::Mul => running * res,
        };
        if rest.starts_with(')') {
            return(new_running, &rest[1..])
        }
        return match rest.get(1..2) {
            Some(op_s) => {
                let next_op = parse_op(op_s);
                eval(&rest[3..], new_running, next_op)
            },
            None => (new_running, rest)
        }

    }
    if lhs.ends_with(')') {
        let fin = lhs.trim_end_matches(')').parse::<i64>().unwrap();
        let res = match run_op {
            Op::Add => running + fin,
            Op::Mul => running * fin,
        };
        return (res, &input[2..])
    }
    if let Some(op) = toks.next() {
        let new_running = match run_op {
            Op::Add => running + lhs.parse::<i64>().unwrap(),
            Op::Mul => running * lhs.parse::<i64>().unwrap(),
        };
        eval(toks.next().unwrap(), new_running, parse_op(op))
    } else {
        let val = match run_op {
            Op::Add => running + lhs.parse::<i64>().unwrap(),
            Op::Mul => running * lhs.parse::<i64>().unwrap(),
        };
        (val, "")
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let input = buffer.trim_end().split("\n");
    let mut sum = 0;
    for line in input {
        let res = eval(line, 0, Op::Add);
        println!("{:?}", res);
        sum += res.0;
    }
    println!("Final: {}", sum);
    

    Ok(())
}
