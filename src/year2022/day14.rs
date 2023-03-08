use nom::{bytes::complete::tag, multi::separated_list1, sequence::tuple, character::complete::u32, IResult, error::{ParseError, Error}};

use crate::day0::Day;

pub struct Day14;

#[derive(Debug)]
struct Point
{
	x : u32,
	y : u32
}

type Rock = Vec<Point>;

#[derive(Debug)]
pub struct RockFormation
{
	parts : Vec<Rock>,
}

impl Day<2022,14,RockFormation,usize> for Day14
{
    fn solve(input: RockFormation) -> usize {
		dbg!(input);
        todo!()
    }

    fn parse(input: &str) -> RockFormation {
		let point_parser = |input| -> IResult<&str,Point,Error<&str>> {
			let (input,x) = nom::sequence::terminated(u32,tag(","))(input)?;
			let (input,y) = u32(input)?;
			Ok((input,Point{x,y}))
		};
    	let parts = separated_list1(tag("\n"),separated_list1(tag(" -> "),point_parser))(input).unwrap().1;
		RockFormation{parts}
    }
}
