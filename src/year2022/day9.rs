use std::{collections::HashSet, error::Error, fmt::Display, str::FromStr};

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
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Default)]
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

        for command in input {
            update_head(&mut head, command);
            update_tail_single(&mut tail, &head, Some(&mut visited));
        }

        visited.len()
    }

    fn solve2(input: Vec<Command>) -> usize {
        const N: usize = 10;
        let mut rope = [Position::default(); N];

        let mut visited = HashSet::new();
        visited.insert(rope[N - 1].clone());
        for mut command in input {
            while command.step > 0 {
                update_head1(&mut rope[0], &command);
                command.step -= 1;
                for i in 0..N - 2 {
                    update_tail(&mut rope, i + 1, i, None);
                }
                update_tail(&mut rope, N - 1, N - 2, Some(&mut visited));
            }

            // buffer[(rope[0].x+500 )as usize][(rope[0].y + 500) as usize] = 'H';
            // for i in 1..N-1
            // {
            //     buffer[(rope[i].x + 500) as usize][(rope[i].y + 500) as usize] = char::from_u32(i as u32).unwrap();
            // }
            // println!("{}",buffer.iter().map(|v| v.iter().collect::<String>()).collect::<Vec<String>>().join("\n"));
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
}

fn update_head1(h: &mut Position, c: &Command) {
    match c.dir {
        Direction::Right => h.x += 1,
        Direction::Down => h.y -= 1,
        Direction::Left => h.x -= 1,
        Direction::Up => h.y += 1,
    }
}

fn update_tail<const N: usize>(
    v: &mut [Position; N],
    it: usize,
    ih: usize,
    mut o_set: Option<&mut HashSet<Position>>,
) {
    loop {
        let x_delta = v[ih].x - v[it].x;
        let y_delta = v[ih].y - v[it].y;
        if x_delta.abs() <= 1 && y_delta.abs() <= 1 {
            break;
        }

        let x_set = x_delta.signum();
        let y_set = y_delta.signum();
        v[it].x += x_set;
        v[it].y += y_set;

        if let Some(ref mut set) = o_set {
            set.insert(v[it].clone());
        }
    }
}
fn update_tail_single(t: &mut Position, h: &Position, mut o_set: Option<&mut HashSet<Position>>) {
    loop {
        let x_delta = h.x - t.x;
        let y_delta = h.y - t.y;
        if x_delta.abs() <= 1 && y_delta.abs() <= 1 {
            break;
        }

        let x_set = x_delta.signum();
        let y_set = y_delta.signum();
        t.x += x_set;
        t.y += y_set;

        if let Some(ref mut set) = o_set {
            set.insert(t.clone());
        }
    }
}
