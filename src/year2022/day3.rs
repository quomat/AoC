use std::collections::HashSet;

use crate::day0::Day;
pub struct Day3;

impl Day<2022, 3,
            Vec<(HashSet<u8>,HashSet<u8>)>,
            u32> 
for Day3
{
    fn parse(input : String) ->  Vec<(HashSet<u8>,HashSet<u8>)>
    {
        input.lines()
                .map(|line| Vec::from(line.as_bytes()))
                .map(|bts| {
                    let mut bts2 = bts.clone();
                    let r = bts2.split_off(bts.len()/2); 
                    (bts2.into_iter().collect(),r.into_iter().collect())
                }).collect()
    }

    fn solve(input : Vec<(HashSet<u8>,HashSet<u8>)>) -> u32 {
        input.into_iter().map(| pair |
        {
            *(pair.0.intersection(&pair.1).next().unwrap() ) 
        }).map(priority).sum()
    }

    fn solve2(input : Vec<(HashSet<u8>,HashSet<u8>)>) -> u32 {
        let mut i = 0;
        let mut sum : u32 = 0;
        while i < input.len()
        {
            let rucksack1 = input[i].0.union(&input[i].1).collect::<HashSet<&u8>>();
            let rucksack2 = input[i+1].0.union(&input[i+1].1).collect::<HashSet<&u8>>();
            let rucksack3 = input[i+2].0.union(&input[i+2].1).collect::<HashSet<&u8>>();
            
            
            let inters = rucksack1.iter().filter(|a1| rucksack2.contains(*a1) && rucksack3.contains(*a1)).collect::<HashSet<&&u8>>();
            assert_eq!(inters.len(),1);
            let emblem = rucksack1.iter().filter(|a1| rucksack2.contains(*a1) && rucksack3.contains(*a1)).next().unwrap();
            
            sum += priority(**emblem);
            
            i += 3;
        }
        
        sum
    }
}

fn priority(c : u8) -> u32
{
    let mut result : u8 = 0;
    if c >= b'a' && c <= b'z' {
        result = c - b'a' + 1;
    } else if c >= b'A' && c <= b'Z' {
        result =  c - b'A' + 27
    }
    
    result.into()
        
        
}