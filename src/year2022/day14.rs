use itertools::Itertools;
use nom::{
    bytes::complete::tag, character::complete::u64, error::Error, multi::separated_list1, IResult,
};
use rand::Rng;
use std::fmt::Debug;
use std::fmt::Write;
use std::{
    collections::VecDeque,
    rc::{Rc, Weak},
};

use crate::day0::Day;

pub struct Day14;

type Coord = u64;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: Coord,
    y: Coord,
}

const SPAWN: Point = Point { x: 500, y: 0 };

type Rock = Vec<Point>;

#[derive(Debug)]
pub struct RockFormation {
    parts: Vec<Rock>,
}

impl Day<2022, 14, RockFormation, usize> for Day14 {
    fn solve(input: RockFormation) -> usize {
        let mut world = World::new();
        for rock in input.parts {
            world.add(Entity::Rock(rock.clone()));
        }

        std::iter::from_fn(|| world.drop()).count()
    }
    fn solve2(input: RockFormation) -> usize {
        let mut world = World::new();
        let y_max = input
            .parts
            .iter()
            .map(|r| r.iter().map(|p| p.y).max().unwrap())
            .max()
            .unwrap()
            + 2;
        for rock in input.parts {
            world.add(Entity::Rock(rock.clone()));
        }
        world.add(Entity::Floor(y_max));

        std::iter::from_fn(|| world.drop()).count()
    }

    fn parse(input: &str) -> RockFormation {
        let point_parser = |input| -> IResult<&str, Point, Error<&str>> {
            let (input, x) = nom::sequence::terminated(u64, tag(","))(input)?;
            let (input, y) = u64(input)?;
            Ok((input, Point { x, y }))
        };
        let parts = separated_list1(tag("\n"), separated_list1(tag(" -> "), point_parser))(input)
            .unwrap()
            .1;
        RockFormation { parts }
    }
}
#[derive(Clone)]
enum Entity {
    Rock(Vec<Point>),
    Sand(Point),
    Floor(Coord),
}

/// Świat składa się z bytów (entity) skeszowanych w tablicy punktów, żeby szybciej wyliczyć kolizje.
struct World {
    entities: Vec<Rc<Entity>>,

    /// Lewy górny róg.
    leftup: Point,
    /// Prawy dolny róg.
    rightdown: Point,
    /// Atomy określają czy pole jest zajęte czy nie
    atoms: VecDeque<VecDeque<Weak<Entity>>>,
}

impl World {
    fn new() -> World {
        World {
            entities: Vec::new(),
            leftup: Point { x: 500, y: 0 },
            rightdown: Point { x: 500, y: 0 },
            atoms: vec![vec![Weak::new()].into()].into(),
        }
    }

    fn add(&mut self, e: Entity) {
        let e = Rc::new(e);
        match e.as_ref() {
            Entity::Sand(s) => self.add_point(s, Rc::downgrade(&e)),
            Entity::Rock(r) => {
                for (line0, line1) in r.iter().tuple_windows() {
                    if line0.x == line1.x {
                        for y in if line0.y < line1.y {
                            line0.y..=line1.y
                        } else {
                            line1.y..=line0.y
                        } {
                            let p = Point { x: line0.x, y };
                            self.add_point(&p, Rc::downgrade(&e));
                        }
                    } else if line0.y == line1.y {
                        for x in if line0.x < line1.x {
                            line0.x..=line1.x
                        } else {
                            line1.x..=line0.x
                        } {
                            let p = Point { x, y: line0.y };
                            self.add_point(&p, Rc::downgrade(&e));
                        }
                    } else {
                        unreachable!()
                    }
                }
            }
            Entity::Floor(y_max) => {
                for x in 0..=self.rightdown.x + 500 {
                    self.add_point(&Point { x, y: *y_max }, Rc::downgrade(&e))
                }
            }
        }

        self.entities.push(e);
    }

    // pub fn width(&self) -> Coord {
    //     self.rightdown.x - self.leftup.x + 1
    // }

    pub fn height(&self) -> Coord {
        self.rightdown.y - self.leftup.y + 1
    }

    fn initialize_weaks(x : usize, y : usize) -> VecDeque<VecDeque<Weak<Entity>>>
    {
        let mut v = Vec::with_capacity(y);
        for _ in 0..y{ v.push(Weak::new())}
        vec![v.into(); x].into()
    }

    fn add_point(&mut self, p: &Point, source: Weak<Entity>) {
        // println!("[World::add_point] Adding point [{:?}] to the world.", p);
        if p.x < self.leftup.x {
            //     println!("[World::add_point] Resizing to the left...");
            let diff = self.leftup.x - p.x;
            let mut new_atoms: VecDeque<VecDeque<Weak<Entity>>> = Self::initialize_weaks(diff as usize, self.height() as usize);
            prepend(&mut self.atoms, &mut new_atoms);
            self.leftup.x = p.x;
        } else if p.x > self.rightdown.x {
            //     println!("[World::add_point] Resizing to the right...");
            let diff = p.x - self.rightdown.x + 1;

            let mut new_atoms: VecDeque<VecDeque<Weak<Entity>>> =
            Self::initialize_weaks(diff as usize, self.height() as usize);
            self.atoms.append(&mut new_atoms);
            self.rightdown.x = p.x;
        }

        if p.y < self.leftup.y {
            //     println!("[World::add_point] Resizing to the up...");
            let diff = self.leftup.y - p.y;
            let new_atoms: VecDeque<Weak<Entity>> = { let mut v = VecDeque::with_capacity(diff as usize); for _ in 0..diff as usize { v.push_back(Weak::new());} v };
            for column in self.atoms.iter_mut() {
                let mut new_column = new_atoms.clone();
                prepend(column, &mut new_column)
            }
            self.leftup.y = p.y;
        } else if p.y > self.rightdown.y {
            //     println!("[World::add_point] Resizing to the down...");
            let diff = p.y - self.rightdown.y + 1;
            let new_atoms: VecDeque<Weak<Entity>> = { let mut v = VecDeque::with_capacity(diff  as usize); for _ in 0..diff  as usize { v.push_back(Weak::new());} v };
            for column in self.atoms.iter_mut() {
                let mut new_column = new_atoms.clone();
                column.append(&mut new_column);
            }
            self.rightdown.y = p.y;
        }
        // println!("[World::add_point] Before add:");
        self.set(p, source);
        // println!("[World::add_point] After add:");
        // println!("{:?}", &self);
    }

    fn set(&mut self, p: &Point, v: Weak<Entity>) {
        let x = self.get_x(p.x).unwrap();
        let y = self.get_y(p.y).unwrap();
        // println!(
        // "[World::set] Setting the world at coordinates ({3},{4})[ rel. {0},{1}] to {2:?}",
        // x, y, v, p.x, p.y
        // );
        self.atoms[x][y] = v;
    }

    fn get_x(&self, x: Coord) -> Option<usize> {
        (x.checked_sub(self.leftup.x)).map(|x| x as usize)
    }

    fn get_y(&self, y: Coord) -> Option<usize> {
        (y.checked_sub(self.leftup.y)).map(|x| x as usize)
    }

    fn get(&self, p: Point) -> Option<Weak<Entity>> {
        let x = self.get_x(p.x)?;
        let y = self.get_y(p.y)?;
        self.atoms
            .get(x)
            .and_then(|col| col.get(y).cloned())
    }

    pub(crate) fn drop(&mut self) -> Option<Entity> {
        let mut pos = SPAWN;

        while self.get(pos).map(|p| p.ptr_eq(&Weak::new())) == Some(true) {
            // println!("{:?}", self);
            let candidates = [
                Point {
                    y: pos.y + 1,
                    ..pos
                },
                Point {
                    y: pos.y + 1,
                    x: pos.x - 1,
                },
                Point {
                    y: pos.y + 1,
                    x: pos.x + 1,
                },
            ];
            let next = candidates
                .into_iter()
                .find(|&p| {
                    self.get(p)
                        .map(|pp| pp.ptr_eq(&Weak::new()))
                        .unwrap_or(true)
                });
            let candidate = next;
            match candidate {
                Some(p) if matches!(self.get(p), None) => return None,
                Some(p) => pos = p,
                None => {
                    let sand = Entity::Sand(pos);
                    self.add(sand.clone());
                    return Some(sand);
                }
            }
        }

        None
    }
}

impl Debug for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut rng = rand::thread_rng();
        if rng.gen_range(0..9000) != 0 {
            // return Ok(());
        }
        let mut s = String::new();
        // write!(
        //     &mut s,
        //     "\nShowing the world\n Trivia: \nwidth = {0} \nheight = {1} \n",
        //     self.width(),
        //     self.height()
        // )?;

        for y in self.leftup.y..=self.rightdown.y {
            for x in self.leftup.x..=self.rightdown.x {
                write!(
                    &mut s,
                    "{}",
                    match self.atoms[self.get_x(x).unwrap()][self.get_y(y).unwrap()].upgrade() {
                        Some(ptr) => match *ptr {
                            Entity::Rock(_) => "#",
                            Entity::Sand(_) => "o",
                            Entity::Floor(_) => "=",
                        },
                        None => ".",
                    }
                )?;
            }
            writeln!(s)?;
        }

        write!(f, "{}c", 27 as char)?;
        // write!(f,"{esc}[2J{esc}[1;1H", esc = 27 as char)?;
        write!(f, "{}", s)?;
        Ok(())
    }
}

fn prepend<T>(deq: &mut VecDeque<T>, other: &mut VecDeque<T>) {
    while let Some(a) = other.pop_back() {
        deq.push_front(a);
    }
}
