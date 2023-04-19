use std::str::FromStr;

use crate::day0::*;

pub struct Day5 {}

type Stack<T> = Vec<T>;

pub struct Move {
    size: usize,
    from: usize,
    to: usize,
}

pub struct Input {
    stacks: Vec<Stack<char>>,
    moves: Vec<Move>,
}

impl Move {
    fn new(s: Option<usize>, f: Option<usize>, t: Option<usize>) -> Option<Move> {
        s.and_then(|sp| {
            f.and_then(|fp| {
                t.and_then(|tp| {
                    Some(Move {
                        size: sp,
                        from: fp - 1,
                        to: tp - 1,
                    })
                })
            })
        })
    }
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut mi = s
            .split(|c: char| c.is_alphabetic() || c.is_whitespace())
            .filter_map(|c| c.parse::<usize>().ok());
        // println!("from_str called on {0}. Split result: {1:?}",s,mi.clone().collect::<Vec<_>>());
        Move::new(mi.next(), mi.next(), mi.next()).ok_or(())
    }
}

impl Day<2022, 5, Input, String> for Day5 {
    fn solve(mut input: Input) -> String {
        for m in input.moves {
            for _ in 0..m.size {
                let v = input.stacks[m.from].pop().unwrap();
                input.stacks[m.to].push(v);
            }
        }
        let mut result = String::new();
        for mut s in input.stacks {
            if let Some(c) = s.pop() {
                result.push(c)
            }
        }
        result
    }

    fn solve2(mut input: Input) -> String {
        for m in input.moves {
            let n = input.stacks[m.from].len();
            let mut v = input.stacks[m.from].split_off(n - m.size);
            input.stacks[m.to].append(&mut v);
        }
        let mut result = String::new();
        for mut s in input.stacks {
            if let Some(c) = s.pop() {
                result.push(c)
            }
        }
        result
    }

    fn parse(input: &str) -> Input {
        let lines: Vec<&str> = input.lines().collect();
        let mut i = 0;
        let mut indices: Vec<u8> = Vec::new();
        while i < lines.len() {
            indices = lines[i]
                .split_whitespace()
                .map(|x| x.parse::<u8>().unwrap_or(0))
                .collect();
            if indices[0] == 1 {
                break;
            }
            i += 1;
        }

        Input {
            stacks: parse_stacks(lines[..i].to_vec(), indices.len()),
            moves: parse_moves(lines[i + 1..].to_vec()),
        }
    }
}

fn parse_moves(input2: Vec<&str>) -> Vec<Move> {
    input2
        .into_iter()
        .filter_map(|l| l.parse::<Move>().ok())
        .collect()
}

fn parse_stacks(input1: Vec<&str>, n: usize) -> Vec<Stack<char>> {
    let mut result_stack: Vec<Stack<char>> = Vec::new();
    let h = input1.len();
    let mut i = h;
    for _ in 0..n {
        result_stack.push(Vec::new());
    }
    while i > 0 {
        let bytes = input1[i - 1].as_bytes();

        debug_assert_eq!(bytes.len(), 4 * n - 1);

        let mut j = 1;
        while j < 4 * n - 1 {
            let c = bytes[j];
            if c.is_ascii_alphabetic() {
                result_stack[j / 4].push(bytes[j] as char);
            }
            j += 4;
        }
        i -= 1;
    }
    // println!("parse_stacks completed. Result: {:?}", result_stack);
    result_stack
}
