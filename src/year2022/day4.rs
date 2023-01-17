use crate::day0::Day;
use std::ops::{RangeInclusive};

pub struct Day4;

impl Day<2022, 4, Vec<(RangeInclusive<u8>, RangeInclusive<u8>)>, u32> for Day4 {
    fn parse(input: &str) -> Vec<(RangeInclusive<u8>, RangeInclusive<u8>)> {
        input
            .lines()
            .map(|line| {
                let mut i = line.split(['-', ',']).map(|n| n.parse::<u8>().unwrap());
                let mut next = || i.next().unwrap();
                (next()..=next(), next()..=next())
            })
            .collect()
    }

    fn solve(input: Vec<(RangeInclusive<u8>, RangeInclusive<u8>)>) -> u32 {
        input.iter().filter(|(r1, r2)| {
            match r1.start().cmp( r2.start() )
            {
                std::cmp::Ordering::Less => {println!("LESS for {:?} {:?}  -- {:?}",r1,r2,r1.end() >= r2.end()); r1.end() >= r2.end()},
                std::cmp::Ordering::Equal => {println!("EQUAL for {:?} {:?}! PASS",r1,r2);true},
                std::cmp::Ordering::Greater => {println!("GREATER for {:?} {:?}. --- {:?}",r1,r2,r1.end() <= r2.end());r1.end() <= r2.end()},
            }
            
        }).count().try_into().unwrap()
    }
    
    fn solve2(input: Vec<(RangeInclusive<u8>, RangeInclusive<u8>)>) -> u32 {
    input.iter().filter(|(r1, r2)| {
            match r1.start().cmp( r2.start() )
            {
                std::cmp::Ordering::Less => {println!("LESS for {:?} {:?}  -- {:?}",r1,r2,r1.end() >= r1.start()); r1.end() >= r2.start()},
                std::cmp::Ordering::Equal => {println!("EQUAL for {:?} {:?}! PASS",r1,r2);true},
                std::cmp::Ordering::Greater => {println!("GREATER for {:?} {:?}. --- {:?}",r1,r2,r1.start() <= r2.end());r1.start() <= r2.end()},
            }
            
        }).count().try_into().unwrap()
    }
}
