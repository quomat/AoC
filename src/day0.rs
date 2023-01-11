use std::{fs, fmt::Display};

const INPUT_FILE_NAME : &str = "input.txt";
pub fn read_input(y : u16, i : u8) -> String
{
    fs::read_to_string(format!("input/year{0}/day{1}/{2}",y,i,INPUT_FILE_NAME)).unwrap()
}

pub trait Day<const Y : u16, const N : u8, I,O> 
where O : Display
{
    fn solve(input : I) -> O;
    fn solve2(input : I) -> O
    {
        Self::solve(input)
    }

    fn parse(input: String) -> I;
    fn parse2(input: String) -> I
    {
        Self::parse(input)
    }

    fn solve_input1() -> O
    {
        Self::solve(Self::parse(read_input(Y,N)))
    }

    fn solve_input2() -> O
    {
        Self::solve2(Self::parse2(read_input(Y,N)))
    }
}