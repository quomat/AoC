use std::str::FromStr;

use nom::{combinator::{map, map_res, opt, recognize, all_consuming}, bytes::complete::tag, sequence::preceded, character::complete::digit1, branch::alt, Finish, error::Error};

use crate::day0::Day;

enum Instruction
{
    Noop,
    AddX(i32)
}

impl Instruction
{
    fn cycles(&self) -> u32
    {
        match self
        {
            Instruction::Noop => 1,
            Instruction::AddX(_) => 2,
        }
    }
}

struct Computer
{
    registerX : i32,

    cycle : u32,
}

impl Computer
{
    fn signal_strength(&self) -> i32
    {
        self.cycle as i32 * self.registerX
    }
}

pub struct Interceptor
{
    counter : i32,
}

impl Interceptor
{
    fn intercept_call(&mut self, c : &Computer)
    {
        match c.cycle
        {
            20 => self.counter += c.signal_strength(),
            60 => self.counter += c.signal_strength(),
            100 => self.counter += c.signal_strength(),
            140 =>self.counter += c.signal_strength(),
            180 => self.counter += c.signal_strength(),
            _ => ()
        }
    }
}


pub struct Day10;

impl Day<2022,10,Vec<Instruction>,i32> for Day10
{
    fn solve(input: Vec<Instruction>) -> i32 {
        let mut comp = Computer { registerX : 1, cycle : 0};
        let mut interceptor = Interceptor { counter : 0};
        for instr in input
        {
            for _j in 0..instr.cycles()
            {
                comp.cycle += 1;
                interceptor.intercept_call(&comp);
            }
        }
        interceptor.counter
    }

    fn parse(input: &str) -> Vec<Instruction> {
        let noop_parser = map(tag::<_,_,Error<_>>("noop"), |_| Instruction::Noop);
        let num_parser = map_res(recognize(preceded(opt(tag("-")), digit1)), i32::from_str);
        
        let addx_parser = map(preceded(tag("addx "), num_parser), |n| Instruction::AddX(n));

        let comm_parser = alt((noop_parser,addx_parser));

        input.lines()
        .map(|l| all_consuming(comm_parser)(l).finish().unwrap().1).collect()
    }
}