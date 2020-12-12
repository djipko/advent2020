use std::collections::VecDeque;
use std::error::Error;
use std::io::{self, Read};

#[derive(Debug, Clone, Copy)]
enum Direction {
    E,
    W,
    N,
    S,
    F,
}

#[derive(Debug, Clone)]
enum Side {
    L,
    R,
}

#[derive(Debug, Clone)]
enum Move {
    Sail(Direction, i32),
    Turn(Side, i32),
}

#[derive(Debug)]
struct Ship {
    x: i32,
    y: i32,
    facing: VecDeque<Direction>,
}

impl Ship {
    fn new() -> Ship {
        Ship {
            x: 0,
            y: 0,
            facing: vec![Direction::E, Direction::N, Direction::W, Direction::S]
                .into_iter()
                .collect(),
        }
    }
    fn parse_move(mov: &str) -> Option<Move> {
        let (d, s) = mov.split_at(1);
        let s = s.parse::<i32>().ok()?;
        match d {
            "E" => Some(Move::Sail(Direction::E, s)),
            "W" => Some(Move::Sail(Direction::W, s)),
            "N" => Some(Move::Sail(Direction::N, s)),
            "S" => Some(Move::Sail(Direction::S, s)),
            "F" => Some(Move::Sail(Direction::F, s)),
            "L" => Some(Move::Turn(Side::L, s)),
            "R" => Some(Move::Turn(Side::R, s)),
            _ => None,
        }
    }

    fn do_move(&mut self, mov: Move) {
        println!("{:?}", mov);
        match mov {
            Move::Sail(d, s) => self.do_sail(d, s),
            Move::Turn(s, a) => self.do_turn(s, a),
        }
    }

    fn do_sail(&mut self, d: Direction, s: i32) {
        match d {
            Direction::E => self.x += s,
            Direction::W => self.x -= s,
            Direction::N => self.y += s,
            Direction::S => self.y -= s,
            Direction::F => self.do_sail(self.facing[0], s),
        }
    }

    fn do_turn(&mut self, s: Side, d: i32) {
        let turn = d / 90;
        match s {
            Side::L => self.facing.rotate_left(turn as usize),
            Side::R => self.facing.rotate_right(turn as usize),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let moves = buffer
        .trim_end()
        .split("\n")
        .map(|m| Ship::parse_move(m))
        .collect::<Option<Vec<Move>>>()
        .ok_or("Failed parsing")?;
    let mut ship = Ship::new();
    for mov in moves.into_iter() {
        ship.do_move(mov);
        println!("{:?}", ship);
    }
    println!("Dist: {}", ship.x.abs() + ship.y.abs());
    Ok(())
}
