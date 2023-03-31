use std::collections::HashSet;

use nom::bytes::complete::tag;

use crate::day0::*;

use self::intervals::IntervalArray;
pub struct Day15<const P: i64>;

impl<const P: i64> Day<2022, 15, Vec<Sensor>, u64> for Day15<P> {
    fn solve(input: Vec<Sensor>) -> u64 {
        Self::sensor_excluded(&input,P,&mut IntervalArray::new())
    }

    fn solve2(input: Vec<Sensor>) -> u64 {
        let r = P * 2; // so it seems
        let mut p = 0.0;
        for i in 0..=r
        {
            let mut ia = IntervalArray::new();
            Self::sensor_excluded(&input,i,&mut ia);
            ia.intersect(0,r);
            
            if let Some(f) = ia.get_first_non()
            {
                return f as u64 * 4000000 + i as u64;
            }
            let pr = (i as f32/r as f32)*100.0;
            if pr > p
            {
                println!("{}%",p);
                p = pr.ceil();
            }
        }
        panic!("sensor not found???");        
    }
    fn parse(input: &str) -> Vec<Sensor> {
        nom::multi::separated_list0(tag("\n"), parsing::sensor)(input)
            .unwrap()
            .1
    }
}

impl<const P:i64> Day15<P>
{
    
    fn sensor_excluded(input: &Vec<Sensor>, k : i64, ia : &mut IntervalArray) -> u64 {
        let mut row_sensors = HashSet::new();
        for sensor in input {
            let d = d(&sensor.pos, &sensor.beacon);
            // println!("[day15] d({0:?},{1:?}) = {2}",sensor.pos,sensor.beacon,d);
            if sensor.beacon.y == k {
                row_sensors.insert(sensor.beacon.x);
            }

            let dd = k.abs_diff(sensor.pos.y);
            // println!("[day15] distance to y = {0} is {1}",P,dd);
            if d >= dd 
            {
                ia.add(
                    sensor.pos.x - (d.saturating_sub(dd) as i64),
                    sensor.pos.x + (d.saturating_sub(dd) as i64),
                );
            }
        }

        ia.count() - (row_sensors.len() as u64)
    }
}
#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
}

fn d(a: &Point, b: &Point) -> u64 {
    a.x.abs_diff(b.x) + a.y.abs_diff(b.y)
}

pub struct Sensor {
    pos: Point,
    /// Closest beacon.
    beacon: Point,
}

mod intervals {
    use itertools::Itertools;

    pub struct IntervalArray {
        switches: Vec<i64>,
    }

    impl IntervalArray {
        pub fn new() -> IntervalArray {
            IntervalArray {
                switches: Vec::new(),
            }
        }

        pub fn add(&mut self, l: i64, r: i64) {
            // println!("[add] Adding [{0}, {1}]",l,r);
            let i = 0;
            let ir = self.switches.partition_point(|&x| x <= r);
            let il = self.switches.partition_point(|&x| x < l);
            let mut insert = Vec::new();
            if il % 2 == 0 {
                insert.push(l);
            }
            if ir % 2 == 0 {
                insert.push(r);
            }
            self.switches.splice(il..ir, insert);
            // println!("[add] After add:");
            // dbg!(&self.switches);
        }

        pub fn count(&self) -> u64 {
            // dbg!(&self.switches);
            self.switches
                .iter()
                .tuples()
                .fold(0, |acc, (&l, &r)| acc + r.abs_diff(l) + 1)
        }

        pub fn invert(&mut self)
        {
            self.switches.pop();
            self.switches.remove(0);
        }

        pub fn intersect(&mut self, l : i64, r : i64)
        {
            // println!("[intersect] intersecting with [{0},{1}]",l,r);
            // dbg!(&self.switches);
            let il = self.switches.partition_point(|&x| x < l);
            // println!("[intersect] il = {0},", il);
            let mut insertl = Vec::new();
            if il % 2 == 1 {
                insertl.push(l);
            }
            // println!("[intersect] before splicing... intertl = {0:?}",insertl);
            self.switches.splice(0..il, insertl);
            // println!("[intersect] l splice done.");
            // println!("[intersect] halfway: {0:?}",self.switches);
            let ir = self.switches.partition_point(|&x| x <= r);
            let mut insertr = Vec::new();
            if ir % 2 == 1 {
                insertr.push(r);
            }
            // println!("[intersect] ir = {0},", ir);
            // println!("[intersect] before splicing... intertr = {0:?}",insertr);
            self.switches.splice(ir.., insertr);
            // println!("[intersect] both splices done :)");
            // println!("[intersect] After intersect:");
            // dbg!(&self.switches);
        }

        pub(crate) fn get_first(&self) -> i64 {
            self.switches[0]
        }

        pub(crate) fn get_first_non(&self) -> Option<i64> {
            for (a,b) in self.switches.iter().skip(1).tuples()
            {
            //     println!("[get_first_non] checking ({0},{1}) for a gap of length 2", a,b);
                if b - a == 2
                {
                    return Some(a+1);
                }
            }
            return None;
        }
    }

    #[cfg(test)]
    mod tests {
        use super::IntervalArray;

        #[test]
        fn add_1() {
            let mut ia = IntervalArray::new();

            ia.add(5, 8);
            ia.add(9, 13);

            assert_eq!(ia.count(), 9);
        }

        #[test]
        fn add_2() {
            let mut ia = IntervalArray::new();

            ia.add(3, 5);
            ia.add(4, 8);
            ia.add(7, 11);

            ia.add(13, 14);

            assert_eq!(ia.count(), 11);
        }
        #[test]
        fn add_3() {
            let mut ia = IntervalArray::new();

            ia.add(-30, -10);
            ia.add(-13, 0);
            ia.add(-3, 10);

            assert_eq!(ia.count(), 41);
        }
        #[test]
        fn add_final() {
            let mut ia = IntervalArray::new();

            ia.add(3, 5);
            ia.add(3, 6); // 4
            ia.add(3, 3);

            ia.add(7, 7); // 1
            ia.add(7, 7);

            ia.add(6, 6);

            ia.add(10, 100); // 91
            ia.add(11, 43);
            ia.add(43, 56);
            ia.add(10, 100);
            ia.add(10, 10);

            assert_eq!(ia.count(), 91 + 1 + 4);
        }
        #[test]
        fn intersect_1() {
            let mut ia = IntervalArray::new();

            ia.add(-30, -10);
            ia.add(-13, 0);
            ia.add(-3, 10);

            assert_eq!(ia.count(), 41);
        }
        #[test]
        fn intersect_2()
        {
            let mut ia = IntervalArray::new();

            ia.add(3,7);
            ia.add(10,13);

            ia.intersect(7,14);
            assert_eq!(ia.count(),5);
        }
    }
}
mod parsing {
    use nom::{
        bytes::complete::tag,
        character::complete::i64,
        sequence::{delimited, preceded},
        IResult,
    };

    use super::*;

    pub(crate) fn sensor(input: &str) -> IResult<&str, Sensor> {
        let (input, pos) = delimited(tag("Sensor at "), point, tag(": "))(input)?;
        let (input, beacon) = preceded(tag("closest beacon is at "), point)(input)?;

        Ok((input, Sensor { pos, beacon }))
    }

    fn point(input: &str) -> IResult<&str, Point> {
        let (input, x) = delimited(tag("x="), i64, tag(", "))(input)?;
        let (input, y) = preceded(tag("y="), i64)(input)?;

        Ok((input, Point { x, y }))
    }

    #[cfg(test)]
    mod tests {
        use nom::Finish;

        use super::sensor;

        #[test]
        fn parse_sensor() {
            let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15";
            let sensor = sensor(input).finish().unwrap().1;

            assert_eq!(sensor.pos.x, 2);
            assert_eq!(sensor.pos.y, 18);
            assert_eq!(sensor.beacon.x, -2);
            assert_eq!(sensor.beacon.y, 15);
        }
    }
}
