use std::cmp::Ordering;
use std::ops::ControlFlow;
use std::str::FromStr;
use std::vec;

use crate::day0::Day;
use itertools::EitherOrBoth::*;
use itertools::Itertools;
use nom::combinator::all_consuming;
use nom::Finish;

pub struct Day13();

impl Day<2022, 13, Vec<(Packet, Packet)>, Vec<usize>> for Day13 {
    fn solve(input: Vec<(Packet, Packet)>) -> Vec<usize> {
        input
            .into_iter()
            .enumerate()
            .filter(|(_, p)| p.0 < p.1)
            .map(|(i, _)| i + 1)
            .collect()
    }

    fn solve2(input: Vec<(Packet, Packet)>) -> Vec<usize> {
        let decoder1 = "[[6]]".parse::<Packet>().unwrap();
        let decoder2 = "[[2]]".parse::<Packet>().unwrap();
        let decoders = vec![decoder1.clone(), decoder2.clone()];

        let mut prepared = input
            .into_iter()
            .flat_map(|(a, b)| vec![a, b])
            .chain(decoders.into_iter())
            .collect::<Vec<Packet>>();

        prepared.sort();

        prepared
            .into_iter()
            .enumerate()
            .filter(|(_, x)| *x == decoder1 || *x == decoder2)
            .map(|(i, _)| i + 1)
            .collect()
    }

    fn answer(output: Vec<usize>) {
        println!("{}", output.iter().sum::<usize>());
    }

    fn answer2(output: Vec<usize>) {
        println!("{}", output.iter().product::<usize>());
    }

    fn parse(input: &str) -> Vec<(Packet, Packet)> {
        let linnes = input.split("\n\n");
        linnes
            .map(|x| {
                let ll = x.lines();
                ll.map(|s| s.parse::<Packet>().unwrap())
                    .next_tuple()
                    .unwrap()
            })
            .collect()
    }
}
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Packet {
    Integer(u32),
    Complex(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use Packet::*;
        // unsafe {
        //     static mut INDENT: usize = 0;
        //     println!(
        //         "{INDENT:indent$}Comapring {0:?} and {1:?}...",
        //         &self,
        //         &other,
        //         indent = INDENT
        //     );
        //     INDENT += 1;

        let res = match (self, other) {
            (Integer(x), Integer(y)) => match x.cmp(y) {
                std::cmp::Ordering::Equal => None,
                other => Some(other),
            },
            (Complex(x), Complex(y)) => {
                match Itertools::zip_longest(x.iter(), y.iter()).try_for_each(|it| match it {
                    Both(p1, p2) => match p1.partial_cmp(p2) {
                        None => ControlFlow::Continue(()),
                        Some(result) => ControlFlow::Break(result),
                    },
                    Left(_) => ControlFlow::Break(Ordering::Greater),
                    Right(_) => ControlFlow::Break(Ordering::Less),
                }) {
                    ControlFlow::Continue(..) => None,
                    ControlFlow::Break(x) => Some(x),
                }
            }
            (&Integer(x), complex) => Complex(vec![Integer(x)]).partial_cmp(complex),
            (complex, &Integer(x)) => complex.partial_cmp(&Complex(vec![Integer(x)])),
        };
        // INDENT -= 1;
        // println!("{INDENT:indent$}... done! {:?}", res,indent = INDENT);
        res
        // }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.partial_cmp(other) {
            Some(order) => order,
            None => Ordering::Equal,
        }
    }
}

impl FromStr for Packet {
    type Err = nom::error::Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match all_consuming(parsers::packet)(s).finish() {
            Ok((_i, a)) => Ok(a),
            Err(nom::error::Error { input, code }) => Err(nom::error::Error {
                input: input.to_owned(),
                code,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use nom::Finish;

    use super::parsers::packet;

    #[test]
    fn packet_comp() {
        let p1 = "[[1],[2,3,4]]";
        let p2 = "[[1],4]";
        let packet1 = packet(p1).finish().unwrap().1;
        let packet2 = packet(p2).finish().unwrap().1;

        assert!(packet1 < packet2);
    }
}
mod parsers {
    use super::*;
    use nom::character::complete as cc;
    use nom::combinator::map;
    use nom::multi::separated_list0;
    use nom::sequence::delimited;
    use nom::{branch::alt, bytes::complete::tag, IResult};

    pub(crate) fn packet(input: &str) -> IResult<&str, Packet> {
        alt((
            map(cc::u32, Packet::Integer),
            map(
                delimited(tag("["), separated_list0(tag(","), packet), tag("]")),
                Packet::Complex,
            ),
        ))(input)
    }

    #[cfg(test)]
    mod tests {
        use nom::Finish;

        use super::*;

        #[test]
        fn packet_1() {
            let p = "[[1],[2,3,4]]";

            let packet = packet(p).finish().unwrap().1;

            assert_eq!(
                packet,
                Packet::Complex(vec![
                    Packet::Complex(vec![Packet::Integer(1)]),
                    Packet::Complex(vec![
                        Packet::Integer(2),
                        Packet::Integer(3),
                        Packet::Integer(4)
                    ])
                ])
            );
        }

        #[test]
        fn packet_2() {
            let p = "[[1],4]";

            let packet = packet(p).finish().unwrap().1;

            assert_eq!(
                packet,
                Packet::Complex(vec![
                    Packet::Complex(vec![Packet::Integer(1)]),
                    Packet::Integer(4)
                ])
            );
        }
        #[test]
        fn packet_trivial() {
            let p = "[]";

            let packet = packet(p).finish().unwrap().1;

            assert_eq!(packet, Packet::Complex(vec![]))
        }
    }
}
