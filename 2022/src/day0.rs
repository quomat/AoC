use std::{fmt::Debug, fmt::Write, fs};

pub trait Answer {
    fn answer(&self) -> String;
}

#[derive(Clone,Copy)]
pub enum Part
{
    Part1,
    Part2
}

pub trait Day<const N: u8, I, O>
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

    fn solve_input(input: &str, part : Part) -> O
    {
        let parsing_fn : Box<dyn Fn(&str) -> I>;
        let solving_fn : Box<dyn Fn(I) -> O>;
        match part {
            Part::Part1 => {parsing_fn =Box::new( Self::parse); solving_fn = Box::new(Self::solve)},
            Part::Part2 => {parsing_fn =Box::new( Self::parse2); solving_fn = Box::new(Self::solve2)},
        }
        let path = format!("input/day{0}/{1}", N, input);
        let res = fs::read_to_string(&path);
        match res {
            Ok(inp) =>  solving_fn(parsing_fn(&inp)),
            Err(_) => panic!("  Error: UPSIIII, nie znaleziono pliku {}. Czy jesteś w folderze 2022?",path),
        }
    }

    fn answer_input(input: &str, part : Part) {
        let answer  = Self::solve_input(input, part);
        let answering_fn : Box<dyn Fn(O)>;

        match part {
        Part::Part1 => answering_fn = Box::new(Self::answer),
        Part::Part2 => answering_fn = Box::new(Self::answer2),
        }
        answering_fn(answer)
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
