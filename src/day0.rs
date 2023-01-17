use std::{fmt::Display, fs};

pub trait Day<const Y: u16, const N: u8, I, O>
where
    O: Display,
{
    fn solve(input: I) -> O;
    fn solve2(input: I) -> O {
        Self::solve(input)
    }

    fn parse(input: &str) -> I;
    fn parse2(input: &str) -> I {
        Self::parse(input)
    }

    fn solve_input1(input: &str) -> O {
        let i = fs::read_to_string(format!("input/year{0}/day{1}/{2}", Y, N, input)).unwrap();
        Self::solve(Self::parse(&i))
    }

    fn solve_input2(input: &str) -> O {
        let i = fs::read_to_string(format!("input/year{0}/day{1}/{2}", Y, N, input)).unwrap();
        Self::solve2(Self::parse2(&i))
    }
}
