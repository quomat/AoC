use std::{fmt::Debug, fmt::Write, fs};

pub trait Answer {
    fn answer(&self) -> String;
}

pub trait Day<const Y: u16, const N: u8, I, O>
where
    O: Answer,
{
    fn solve(input: I) -> O;
    fn solve2(input: I) -> O {
        Self::solve(input)
    }

    fn parse(input: &str) -> I;
    fn parse2(input: &str) -> I {
        Self::parse(input)
    }

    fn answer(output: O) {
        println!("{:}", output.answer());
    }

    fn answer2(output: O) {
        println!("{:}", output.answer());
    }

    fn solve_input1(input: &str) -> O {
        let i = fs::read_to_string(format!("input/year{0}/day{1}/{2}", Y, N, input)).unwrap();
        Self::solve(Self::parse(&i))
    }

    fn solve_input2(input: &str) -> O {
        let i = fs::read_to_string(format!("input/year{0}/day{1}/{2}", Y, N, input)).unwrap();
        Self::solve2(Self::parse2(&i))
    }

    fn answer_input1(input: &str) {
        Self::answer(Self::solve_input1(input));
    }

    fn answer_input2(input: &str) {
        Self::answer2(Self::solve_input2(input));
    }
}

impl<T> Answer for T
where
    T: Debug,
{
    fn answer(&self) -> String {
        let mut s = String::new();
        write!(s, "{:?}", self).unwrap();
        s
    }
}
