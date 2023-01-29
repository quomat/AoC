use std::{
    collections::{HashMap, HashSet},
    default,
    error::Error,
    fmt::Display,
    num::ParseIntError,
    str::FromStr,
};

use crate::day0::Day;

#[derive(Debug)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

#[derive(Debug)]
struct DirectionParseError;

impl Display for DirectionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Direction parse error. Excepted \"R\", \"D\", \"L\" or \"U\""
        )
    }
}

impl Error for DirectionParseError {}
impl FromStr for Direction {
    type Err = DirectionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Direction::Right),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "U" => Ok(Direction::Up),
            _ => Err(DirectionParseError),
        }
    }
}

#[derive(Debug)]
pub struct Command {
    dir: Direction,
    step: i32,
}

impl FromStr for Command {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let terms = s.split_whitespace().collect::<Vec<&str>>();
        if terms.len() != 2 {
            return Err(Box::new(DirectionParseError));
        }
        let d = terms[0].parse::<Direction>()?;
        let s = terms[1].parse::<i32>()?;

        Ok(Command { dir: d, step: s })
    }
}
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Position {
    x: i32,
    y: i32,
}



pub struct Day9;

impl Day<2022, 9, Vec<Command>, usize> for Day9 {
    fn solve(input: Vec<Command>) -> usize {
        let mut head = Position { x: 0, y: 0 };
        let mut tail = Position { x: 0, y: 0 };

        let mut visited = HashSet::new();
        visited.insert(tail.clone());
        
        dbg!(&input);

        for command in input {
            update_head(&mut head, command);
            update_tail(&mut tail, &head, &mut visited);
        }

        visited.len()
    }

    fn parse(input: &str) -> Vec<Command> {
        input
            .lines()
            .map(Command::from_str)
            .map(Result::unwrap)
            .collect()
    }
}

fn update_head(h: &mut Position, c: Command) {
    match c.dir {
        Direction::Right => h.x += c.step,
        Direction::Down => h.y -= c.step,
        Direction::Left => h.x -= c.step,
        Direction::Up => h.y += c.step,
    }
    dbg!(&h);
}

fn update_tail(t: &mut Position, h: &Position, set: &mut HashSet<Position>) {
    loop {
        let x_delta = h.x - t.x;
        let y_delta = h.y - t.y;
        if x_delta == 0 || y_delta == 0 {
            break;
        }

        let x_set = x_delta.signum();
        let y_set = y_delta.signum();
        t.x += x_set;
        t.y += y_set;
        dbg!(&t);
        set.insert(t.clone());
    }
}
