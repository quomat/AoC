use nom::{bytes::complete::tag, multi::separated_list1, sequence::tuple, character::complete::u32};

use crate::day0::Day;

struct Day14;

struct Point
{
	x : u32,
	y : u32
}

type Rock = Vec<Point>;

struct RockFormation
{
	parts : Vec<Rock>,
}

impl Day<2022,14,RockFormation,usize> for Day14
{
    fn solve(input: I) -> O {
        todo!()
    }

    fn parse(input: &str) -> RockFormation {
		let arrow = tag(" -> ");
    	let formation = separated_list1(arrow,tuple(u32,tag(","),u32));
    }
}
