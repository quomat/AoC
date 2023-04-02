use nom::{
    combinator::all_consuming,
    error::ErrorKind,
    multi::{separated_list0, separated_list1},
};
use nom_supreme::final_parser::final_parser;

use crate::day0::Day;

pub struct Day16;

impl Day<2022, 16, Vec<Valve>, u64> for Day16 {
    fn solve(input: Vec<Valve>) -> u64 {
        dbg!(input);
        
    }

    fn parse(input: &str) -> Vec<Valve> {
        input
            .lines()
            .map(|i| {
                final_parser::<&str, Valve, (&str, ErrorKind), (&str, ErrorKind)>(parsing::valve)(i)
                    .unwrap()
            })
            .collect()
    }
}

type ValveIndex = [u8;2];

#[derive(Debug)]
pub struct Valve {
    idx: ValveIndex,
    flow_rate: u32,
    leads: Vec<String>,
}

mod parsing {
    use std::fmt;
    use super::Valve;
    use nom::branch::alt;
    use nom::bytes::complete::take;
    use nom::character::complete::alpha1;
    use nom::character::complete::u32;
    use nom::combinator::map;
    use nom::error::ErrorKind;
    use nom::multi::separated_list0;
    use nom::sequence::{preceded, tuple};
    use nom::{IResult, Parser};
    use nom_supreme::tag::complete::tag;

    pub(crate) fn valve(input: &str) -> IResult<&str, Valve, (&str, ErrorKind)> {
        map(
            tuple((
                preceded(tag("Valve "), map(take(2usize),|x : &str| x.bytes().try_into().unwrap())),
                preceded(tag(" has flow rate="), u32),
                preceded(
                    alt((
                        tag("; tunnels lead to valves "),
                        tag("; tunnel leads to valve "),
                    )),
                    separated_list0(tag(", "), alpha1.map(String::from)),
                ),
            )),
            |(idx, flow_rate, leads)| Valve {
                idx,
                flow_rate,
                leads,
            },
        )(input)
    }
}
