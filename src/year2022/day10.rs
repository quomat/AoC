use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{all_consuming, map, map_res, opt, recognize},
    error::Error,
    sequence::preceded,
};

use crate::day0::Day;

#[derive(Debug)]
pub enum Instruction {
    Noop,
    AddX(i32),
}

impl Instruction {
    fn cycles(&self) -> u32 {
        match self {
            Instruction::Noop => 1,
            Instruction::AddX(_) => 2,
        }
    }
}

struct Computer {
    register_x: i32,

    cycle: u32,
}

impl Computer {
    fn signal_strength(&self) -> i32 {
        self.cycle as i32 * self.register_x
    }

    fn compute(&mut self, i: &Instruction) {
        match i {
            Instruction::Noop => (),
            Instruction::AddX(x) => self.register_x += x,
        }
    }
}

pub struct Interceptor {
    counter: i32,
}

impl Interceptor {
    fn intercept_call(&mut self, c: &Computer) {
        match c.cycle {
            20 => self.counter += c.signal_strength(),
            60 => self.counter += c.signal_strength(),
            100 => self.counter += c.signal_strength(),
            140 => self.counter += c.signal_strength(),
            180 => self.counter += c.signal_strength(),
            _ => (),
        }
    }
}

pub struct Day10;

impl Day<2022, 10, Vec<Instruction>, i32> for Day10 {
    fn solve(input: Vec<Instruction>) -> i32 {
        dbg!(&input);
        let mut comp = Computer {
            register_x: 1,
            cycle: 0,
        };
        let mut interceptor = Interceptor { counter: 0 };
        for instr in input {
            for _j in 0..instr.cycles() {
                comp.cycle += 1;
                interceptor.intercept_call(&comp);
            }
            comp.compute(&instr);
        }
        interceptor.counter
    }

    fn parse(input: &str) -> Vec<Instruction> {
        let noop_parser = map(tag::<_, _, Error<_>>("noop"), |_| Instruction::Noop);
        let num_parser = map_res(recognize(preceded(opt(tag("-")), digit1)), i32::from_str);

        let addx_parser = map(preceded(tag("addx "), num_parser), |n| Instruction::AddX(n));

        let comm_parser = all_consuming(alt((noop_parser, addx_parser)));

        input
            .lines()
            .map(comm_parser)
            .map(nom::Finish::finish)
            .map(Result::unwrap)
            .map(|r| r.1)
            .collect()
    }
}
