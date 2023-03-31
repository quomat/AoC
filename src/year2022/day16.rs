
struct Day16;


struct Valve
{
	idx : String,
	flow_rate : u32,
	leads : Vec<String>
}

mod parsing
{
    use nom::IResult;
    use nom::character::complete::alpha1;
    use nom_supreme::tag::complete::tag;
	use nom_supreme::parser_ext::ParserExt;

    use super::Valve;

	fn valve(input : &str) -> IResult<&str, Valve>
	{
		let (input, _) = tag("Value ")(input)?;
		let (input, idx) = alpha1(input)?;
	}
}
