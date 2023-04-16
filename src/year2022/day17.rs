use std::{cmp::max, ops};

use crate::day0::Day;

pub struct Day17;

#[derive(Debug, Clone, Copy)]
pub enum Move {
    Left,
    Right,
}

impl TryFrom<char> for Move {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '>' => Ok(Move::Right),
            '<' => Ok(Move::Left),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: u64,
    y: u64,
}
#[derive(Debug, Clone, Copy)]
struct Vector {
    x: u64,
    y: u64,
}

#[derive(Clone, Copy)]
struct Shape(&'static [Vector]);

const SHAPES: &[Shape] = &[
    Shape(&[
        Vector { x: 0, y: 0 },
        Vector { x: 1, y: 0 },
        Vector { x: 2, y: 0 },
        Vector { x: 3, y: 0 },
    ]),
    Shape(&[
        Vector { x: 0, y: 1 },
        Vector { x: 1, y: 0 },
        Vector { x: 1, y: 1 },
        Vector { x: 2, y: 1 },
        Vector { x: 1, y: 2 },
    ]),
    Shape(&[
        Vector { x: 0, y: 0 },
        Vector { x: 1, y: 0 },
        Vector { x: 2, y: 0 },
        Vector { x: 2, y: 1 },
        Vector { x: 2, y: 2 },
    ]),
    Shape(&[
        Vector { x: 0, y: 0 },
        Vector { x: 0, y: 1 },
        Vector { x: 0, y: 2 },
        Vector { x: 0, y: 3 },
    ]),
    Shape(&[
        Vector { x: 0, y: 0 },
        Vector { x: 1, y: 0 },
        Vector { x: 0, y: 1 },
        Vector { x: 1, y: 1 },
    ]),
];

impl ops::Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Clone, Copy)]
struct EmbeddedShape {
    p: Point,
    shape: Shape,
}

fn intersect(s1: EmbeddedShape, s2: EmbeddedShape) -> bool {
    for &v1 in s1.shape.0 {
        for &v2 in s2.shape.0 {
            if s1.p + v1 == s2.p + v2 {
                //     println!("intersection found! vertex {0:?} from the shape at point {1:?} meets with the vertex {2:?} form the shape at point {3:?}",v1,s1.p,v2,s2.p);
                return true;
            }
        }
    }
    return false;
}

fn tetris<F>(width: u64, n: usize, moves: Vec<Move>, draw: F) -> u64
where
    F: Fn(&[EmbeddedShape], &EmbeddedShape, u64),
{
    let mut world = Vec::new();
    let mut h = 0;
    let mut progress = 0.0;
    let mut j = 0;
    for i in 0..n {
        if (i as f64) / (n as f64) > progress + 0.00000001 {
            progress = (100.0 * i as f64) / (n as f64);
            println!("{0}%", progress);
        }
        let shape = SHAPES[i % SHAPES.len()];
        let mut p = Point { x: 3, y: h + 4 }; // powinno być 3 ale
        let mut jet = false; // zaczynamy od spadania 1 w dół
        'fall: loop {
            jet = !jet;
            if h < 20 {
                draw(&world, &EmbeddedShape { p, shape }, h + 10);
                if jet {
                    //         dbg!(&moves[j % moves.len()]);
                } else {
                    //     println!("Down");
                }
            }
            let pn = do_move(jet, moves[j % moves.len()], p);
            if jet {
                j += 1;
            }
            let curr = EmbeddedShape { p: pn, shape };
            for emb in world.iter().rev().take(17) {
                // heura
                if intersect(*emb, curr) {
                    if !jet {
                        //         println!("fatal: block impact");
                        break 'fall;
                    } else {
                        //             println!("fail: block");
                        continue 'fall;
                    }
                }
            }
            if pn.x == 0 {
                //     println!("fail: left wall");
                continue 'fall;
            }
            if pn.x + max_x(shape) > width {
                //     println!("fail: right wall");
                continue 'fall;
            }
            if pn.y == 0 {
                //     println!("fatal: floor impact");
                break 'fall;
            }

            p = pn;
        }
        world.push(EmbeddedShape { p, shape });
        h = max(h, p.y + max_y(shape));
        if h < 20 {
            // dbg!(h);
            // dbg!(&p);
        }
    }

    h
}

fn max_x(shape: Shape) -> u64 {
    shape.0.iter().map(|p| p.x).max().unwrap()
}

fn max_y(shape: Shape) -> u64 {
    shape.0.iter().map(|p| p.y).max().unwrap()
}

fn do_move(jet: bool, j: Move, p: Point) -> Point {
    if !jet {
        Point { y: p.y - 1, ..p }
    } else {
        match j {
            Move::Left => Point { x: p.x - 1, ..p },
            Move::Right => Point { x: p.x + 1, ..p },
        }
    }
}

impl Day<2022, 17, Vec<Move>, u64> for Day17 {
    fn solve(input: Vec<Move>) -> u64 {
        const W: u64 = 7;
        let draw = |world: &[EmbeddedShape], curr: &EmbeddedShape, h: u64| {
            #[derive(Clone, Copy)]
            enum State {
                Blank,
                Stale,
                Falling,
            }
            fn state_str(s: &State) -> &str {
                match s {
                    State::Blank => ".",
                    State::Stale => "#",
                    State::Falling => "@",
                }
            }
            let w = W + 2;
            let mut board = vec![State::Blank; (h * w) as usize];
            for e in world {
                for &v in e.shape.0 {
                    let p = e.p + v;
                    board[(w * (p.y - 1) + (p.x)) as usize] = State::Stale;
                }
            }
            for &v in curr.shape.0 {
                let p = curr.p + v;
                board[(w * (p.y - 1) + (p.x)) as usize] = State::Falling;
            }
            let mut s = String::new();
            for y in 0..h {
                for x in 0..w {
                    if x == 0 || x == w - 1 {
                        s.push_str("|");
                    } else {
                        s.push_str(state_str(&board[(w * (h - y - 1) + x) as usize]));
                    }
                }
                if y != h - 1 {
                    s.push_str("\n");
                }
            }
            // println!("{}",s);
            // println!("+-------+");
        };

        tetris(W, 2022, input, draw)
    }

    fn solve2(input: Vec<Move>) -> u64 {
        tetris(7, 1000000000000, input, |_, _, _| {})
    }

    fn parse(input: &str) -> Vec<Move> {
        input
            .chars()
            .map(Move::try_from)
            .map(Result::unwrap)
            .collect()
    }
}
