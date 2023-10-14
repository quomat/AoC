use std::{
    fmt::Display,
    io::{self, BufRead},
    str::FromStr,
};

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
    crt: Vec<char>,
}

impl Interceptor {
    fn intercept_call(&mut self, c: &Computer) {
        match c.cycle {
            20 | 60 | 100 | 140 | 180 | 220 => self.counter += c.signal_strength(),
            _ => (),
        }
        if c.cycle % 40 == 1 && c.cycle > 1 {
            self.crt.push('\n');
        }

        if ((c.cycle as i32 - 1) % 40 - c.register_x).abs() <= 1 {
            self.crt.push('#')
        } else {
            self.crt.push('.')
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum ComputerOutput {
    SignalSum(i32),
    Screen(String),
}

impl Display for ComputerOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComputerOutput::SignalSum(x) => write!(f, "{}", x),
            ComputerOutput::Screen(y) => write!(f, "{}", y),
        }
    }
}

pub struct Day10;

impl Day<2022, 10, Vec<Instruction>, ComputerOutput> for Day10 {
    fn solve(input: Vec<Instruction>) -> ComputerOutput {
        ComputerOutput::SignalSum(play(input).counter)
    }

    fn solve2(input: Vec<Instruction>) -> ComputerOutput {
        ComputerOutput::Screen(play(input).crt.into_iter().collect())
    }

    fn answer2(output: ComputerOutput) {
        match output {
            ComputerOutput::SignalSum(_) => unreachable!(),
            ComputerOutput::Screen(x) => println!("{}", x),
        }
        println!("Does this emulated CRT screen show 8 characters? (y/n)");

        let stdin = io::stdin();
        let mut lines = stdin.lock().lines();
        match lines.next() {
            Some(Ok(b)) if b == "y" => println!("Good"),
            _ => panic!(),
        }
    }

    fn parse(input: &str) -> Vec<Instruction> {
        let noop_parser = map(tag::<_, _, Error<_>>("noop"), |_| Instruction::Noop);
        let num_parser = map_res(recognize(preceded(opt(tag("-")), digit1)), i32::from_str);

        let addx_parser = map(preceded(tag("addx "), num_parser), Instruction::AddX);

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

fn play(input: Vec<Instruction>) -> Interceptor {
    let mut comp = Computer {
        register_x: 1,
        cycle: 0,
    };
    let mut interceptor = Interceptor {
        counter: 0,
        crt: Vec::new(),
    };
    for instr in input {
        for _j in 0..instr.cycles() {
            comp.cycle += 1;
            interceptor.intercept_call(&comp);
        }
        comp.compute(&instr);
    }
    interceptor
}
