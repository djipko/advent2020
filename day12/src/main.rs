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


#[derive(Debug, Clone, Copy)]
enum Direction2 {
    E,
    W,
    N,
    S,
}

#[derive(Debug, Clone)]
enum Move2 {
    MoveWp(Direction2, i32),
    TurnWp(Side, i32),
    Sail(i32),
}

#[derive(Debug)]
struct Ship2 {
    x: i32,
    y: i32,
    rwpx: i32,
    rwpy: i32,
}

impl Ship2 {
    fn new() -> Ship2 {
        Ship2 {x: 0, y: 0, rwpx: 10, rwpy: 1}
    }

    fn parse_move(mov: &str) -> Option<Move2> {
        let (d, s) = mov.split_at(1);
        let s = s.parse::<i32>().ok()?;
        match d {
            "E" => Some(Move2::MoveWp(Direction2::E, s)),
            "W" => Some(Move2::MoveWp(Direction2::W, s)),
            "N" => Some(Move2::MoveWp(Direction2::N, s)),
            "S" => Some(Move2::MoveWp(Direction2::S, s)),
            "F" => Some(Move2::Sail(s)),
            "L" => Some(Move2::TurnWp(Side::L, s)),
            "R" => Some(Move2::TurnWp(Side::R, s)),
            _ => None,
        }
    }

    fn do_move(&mut self, mov: Move2) {
        println!("{:?}", mov);
        match mov {
            Move2::Sail(s) => self.do_sail(s),
            Move2::MoveWp(d, s) => self.move_wp(d, s),
            Move2::TurnWp(s, a) => self.turn_wp(s, a),
        }
    }
    
    fn do_sail(&mut self, s: i32) {
        self.x = self.x + s * self.rwpx;
        self.y = self.y + s * self.rwpy;
    }

    fn move_wp(&mut self, d: Direction2, s: i32) {
        match d {
            Direction2::E => self.rwpx += s,
            Direction2::W => self.rwpx -= s,
            Direction2::N => self.rwpy += s,
            Direction2::S => self.rwpy -= s,
        }
    }
    
    fn turn_wp(&mut self, s: Side, a: i32) {
        let times = a / 90;
        for _ in 0..times {
            let old_rwpx = self.rwpx;
            let old_rwpy = self.rwpy;
            match s {
                Side::L => {
                    self.rwpx = -old_rwpy;
                    self.rwpy = old_rwpx;
                }
                Side::R => {
                    self.rwpy = -old_rwpx;
                    self.rwpx = old_rwpy;
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let moves = buffer
        .trim_end()
        .split("\n");
    let moves1 = moves.clone()
        .map(|m| Ship::parse_move(m))
        .collect::<Option<Vec<Move>>>()
        .ok_or("Failed parsing")?;
    let mut ship = Ship::new();
    for mov in moves1.into_iter() {
        ship.do_move(mov);
        println!("{:?}", ship);
    }
    println!("Dist: {}", ship.x.abs() + ship.y.abs());
    let moves2 = moves
        .map(|m| Ship2::parse_move(m))
        .collect::<Option<Vec<Move2>>>()
        .ok_or("Failed parsing")?;
    let mut ship = Ship2::new();
    for mov in moves2.into_iter() {
        ship.do_move(mov);
        println!("{:?}", ship);
    }
    println!("Dist: {}", ship.x.abs() + ship.y.abs());
    Ok(())
}
