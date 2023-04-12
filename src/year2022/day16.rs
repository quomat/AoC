use std::collections::{HashMap, HashSet};

use nom::error::Error;
use nom_supreme::final_parser::final_parser;

use crate::day0::Day;

pub struct Day16<const m : u16>;

impl<const m : u16> Day<2022, 16, Vec<Valve>, u64> for Day16<m> {
    fn solve(input: Vec<Valve>) -> u64 {
        let va = ValveArena::new(input);
        dbg!(&va);
        let mut r = m;
        let mut current : ValveIndex = ['A','A'];
        let mut water = 0;
        let mut opened = HashSet::<ValveIndex>::new();
        while r > 0
        {
        }
        
        water
    }

    fn parse(input: &str) -> Vec<Valve> {
        input
            .lines()
            .map(|i| {
                final_parser::<&str, Valve, Error<&str> , Error<&str>>(parsing::valve)(i)
                    .unwrap()
            })
            .collect()
    }
}
#[derive(Debug)]
struct ValveArena
{
    valves : HashMap<ValveIndex,Valve>,
    paths : HashMap<ValveIndex,HashMap<ValveIndex,u16>>
}

impl ValveArena
{
    fn new(valves : Vec<Valve>) -> ValveArena
    {
        
        let mut map = HashMap::new();
        let paths = HashMap::new();
        for valve in valves
        {
            map.insert(valve.idx,valve);
        }
        
        let mut va = ValveArena{valves:map, paths};
        va.explore();

        va
    }
    fn explore(&mut self)
    {
        for idx in self.valves.keys()
        {
            
        
            let mut batch = Vec::new();
            batch.push(*idx);
            let mut step = 0;
            while !batch.is_empty()
            {
                let next = batch.clone();
                batch = Vec::new();
                for f in next
                {
                            self.paths.entry(*idx).and_modify(|map| {map.insert(f,step);}).or_insert(HashMap::new());
                            for neighbour in &self.valves.get(&f).unwrap().leads
                            {
                                if let Some(n) = self.paths.get(idx)
                                {
                                    if n.contains_key(neighbour){ 
                                    batch.push(*neighbour);    
                                    }
                                }
                            }
                        }
                step += 1;
            }
                    }
    }
}


type ValveIndex = [char;2];

#[derive(Debug)]
pub struct Valve {
    idx: ValveIndex,
    flow_rate: u64,
    leads: Vec<ValveIndex>,
}

mod parsing {
    
    use super::Valve;
    use super::ValveIndex;
    use nom::branch::alt;
    
    
    use nom::character::complete::anychar;
    
    use nom::character::complete::u64;
    use nom::combinator::map;
    
    use nom::multi::separated_list0;
    use nom::sequence::pair;
    use nom::sequence::{preceded, tuple};
    use nom::IResult;
    use nom_supreme::tag::complete::tag;

    fn idx(input: &str) -> IResult<&str, ValveIndex>
    {
        map( pair(anychar,anychar), |(c1,c2)| [c1,c2])(input)
    }

    pub(crate) fn valve(input: &str) -> IResult<&str, Valve> {
        map(
            tuple((
                preceded(tag("Valve "), idx),
                preceded(tag(" has flow rate="), u64),
                preceded(
                    alt((
                        tag("; tunnels lead to valves "),
                        tag("; tunnel leads to valve "),
                    )),
                    separated_list0(tag(", "), idx),
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
