use nom::bytes::complete::tag;

use crate::day0::*;

use self::intervals::IntervalArray;
pub struct Day15<const P : i32>;



impl<const P : i32> Day<2022,15,Vec<Sensor>, u32>  for Day15<P>
{
    fn solve(input: Vec<Sensor>) -> u32 {
		let mut ia = IntervalArray::new();
		let mut row_sensor_count = 0;
		for sensor in input
		{
			let d = d(&sensor.pos,&sensor.beacon);
			if sensor.beacon.y == P
			{ row_sensor_count += 1;}
			
			let dd = P.abs_diff(sensor.pos.y);
			
			ia.add(sensor.pos.x - (d.saturating_sub(dd) as i32),sensor.pos.x + (d.saturating_sub(dd) as i32));
		}

		ia.count() - row_sensor_count
		
    }


    fn parse(input: &str) -> Vec<Sensor> {
		nom::multi::separated_list0(tag("\n"),parsing::sensor)(input).unwrap().1
    }
}

struct Point
{
	x : i32,
	y : i32,
}

fn d(a : &Point, b : &Point) -> u32
{
	a.x.abs_diff(b.x) + a.y.abs_diff(b.y)
}

pub struct Sensor
{
	pos : Point,
	/// Closest beacon.
	beacon : Point
}


mod intervals
{
    use itertools::Itertools;

	

	pub struct IntervalArray
	{
		switches : Vec<i32>,
	}

	impl IntervalArray
	{
		pub fn new() -> IntervalArray
		{
			IntervalArray { switches: Vec::new() }
		}
		
		pub fn add(&mut self, l : i32, r : i32)
		{
			let i = 0;
			let ir = self.switches.partition_point(|&x| x <= r);
			let il = self.switches.partition_point(|&x| x < l);
			let mut insert = Vec::new();
			if il % 2 == 0 { insert.push(l);}
			if ir % 2 == 0 { insert.push(r);}
			self.switches.splice(il..ir,insert);
		}

		pub fn count(&self) -> u32
		{
			dbg!(&self.switches);
			self.switches.iter().tuples().fold(0,|acc,(&l,&r)| acc + r.abs_diff(l) + 1)
		}
	}

	#[cfg(test)]
	mod tests
	{
    use super::IntervalArray;

		#[test]
		fn add_1()
		{
			let mut ia = IntervalArray::new();

			ia.add(5,8);
			ia.add(9,13);

			assert_eq!(ia.count(),9);
		}

		#[test]
		fn add_2()
		{
			let mut ia = IntervalArray::new();

			ia.add(3,5);
			ia.add(4,8);
			ia.add(7,11);

			ia.add(13,14);

			assert_eq!(ia.count(),11);
		}
		#[test]
		fn add_final()
		{
			let mut ia = IntervalArray::new();

			ia.add(3,5);
			ia.add(3,6); // 4
			ia.add(3,3);
			
			ia.add(7,7); // 1
			ia.add(7,7);
			
			ia.add(6,6);
			
			ia.add(10,100); // 91 
			ia.add(11,43);
			ia.add(43,56);
			ia.add(10,100);
			ia.add(10,10);

			assert_eq!(ia.count(),91+1+4);
		}
	}
}
mod parsing
{
    use nom::{IResult, sequence::{preceded, delimited}, bytes::complete::tag, character::complete::i32};

    use super::*;

	pub(crate) fn sensor(input : &str) -> IResult<&str, Sensor>
	{	
		let (input, pos) = delimited(
			tag("Sensor at "),
			point,
			tag(": "),
		)(input)?;
		let (input, beacon) = preceded(
			tag("closest beacon is at "),
			point
		)(input)?;

		Ok((input,Sensor{pos,beacon}))
	}

	fn point(input : &str) -> IResult<&str,Point>
	{
	
		let (input,x) = delimited(
			tag("x="),
			i32,
			tag(", "),
		)(input)?;
		let (input,y) = preceded(
			tag("y="),
			i32
		)(input)?;

		Ok((input,Point{x,y}))
		
	}

	#[cfg(test)]
	mod tests
	{
    use nom::Finish;

    use super::sensor;

		#[test]
		fn parse_sensor()
		{
			let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15";
			let sensor = sensor(input).finish().unwrap().1;
			
			assert_eq!(sensor.pos.x,2);
			assert_eq!(sensor.pos.y,18);
			assert_eq!(sensor.beacon.x,-2);
			assert_eq!(sensor.beacon.y,15);
			
		}
	}
	
}