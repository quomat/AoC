use std::collections::{HashMap, HashSet};

use nom::error::Error;
use nom_supreme::final_parser::final_parser;

use crate::day0::Day;

pub struct Day16<const M: u16>;

type CurrentSolveState = Vec<(
    (ValveIndex, u16),
    u64,
    HashSet<ValveIndex>,
    (ValveIndex, u16),
)>;

impl<const M: u16> Day<16, Vec<Valve>, u64> for Day16<M> {
    fn solve(input: Vec<Valve>) -> u64 {
        let va = ValveArena::new(input);
        // dbg!(&va);
        let mut current: Vec<(ValveIndex, u64, HashSet<ValveIndex>, u16)> =
            vec![(['A', 'A'], 0, va.valves.clone().into_keys().collect(), M)];
        let mut max = 0;
        while !current.is_empty() {
            let mut new_current = Vec::new();
            dbg!(&current.len());
            for c in current.iter() {
                if c.1 > max {
                    max = c.1
                }
                for d in c.2.iter() {
                    let path_length = va.paths[&c.0][d];
                    let rem = c.3.saturating_sub(path_length + 1);
                    if rem == 0 || va.valves[d].flow_rate == 0 {
                        continue;
                    }
                    let pressure = rem as u64 * va.valves[d].flow_rate;
                    let mut targets: HashSet<ValveIndex> = c.2.clone();
                    targets.remove(d);

                    let new = (*d, c.1 + pressure, targets, rem);
                    new_current.push(new);
                }
            }
            current = new_current;
        }
        max
    }

    fn solve2(input: Vec<Valve>) -> u64 {
        let va = ValveArena::new(input);
        // dbg!(&va);
        let mut current: CurrentSolveState = vec![(
            (['A', 'A'], M),
            0,
            va.valves
                .clone()
                .into_keys()
                .filter(|val| va.valves[val].flow_rate != 0)
                .collect(),
            (['A', 'A'], M),
        )];
        let mut max = 0;
        while !current.is_empty() {
            let mut new_current = Vec::new();
            dbg!(&current.len());
            dbg!(&current.first().map(|v| v.2.len()));
            for c in current.iter() {
                if c.1 > max {
                    max = c.1
                }
                for d in c.2.iter() {
                    let path_length = va.paths[&c.0 .0][d];
                    let rem = c.0 .1.saturating_sub(path_length + 1);
                    if rem == 0 || va.valves[d].flow_rate == 0 {
                        continue;
                    }
                    let pressure = rem as u64 * va.valves[d].flow_rate;
                    let mut targets: HashSet<ValveIndex> = c.2.clone();
                    targets.remove(d);
                    let new = (c.3, c.1 + pressure, targets, (*d, rem));
                    if c.1 + pressure > 3 * (max / 4) {
                        new_current.push(new);
                    }
                }
            }
            current = new_current;
        }
        max
    }
    fn parse(input: &str) -> Vec<Valve> {
        input
            .lines()
            .map(|i| {
                final_parser::<&str, Valve, Error<&str>, Error<&str>>(parsing::valve)(i).unwrap()
            })
            .collect()
    }
}
#[derive(Debug)]
struct ValveArena {
    valves: HashMap<ValveIndex, Valve>,
    paths: HashMap<ValveIndex, HashMap<ValveIndex, u16>>,
}

impl ValveArena {
    fn new(valves: Vec<Valve>) -> ValveArena {
        let mut map = HashMap::new();
        let paths = HashMap::new();
        for valve in valves {
            map.insert(valve.idx, valve);
        }

        let mut va = ValveArena { valves: map, paths };
        va.explore();

        va
    }
    fn explore(&mut self) {
        for idx in self.valves.keys() {
            let mut batch = vec![*idx];
            let mut step = 0;
            while !batch.is_empty() {
                let next = batch.clone();
                batch = Vec::new();
                for f in next {
                    self.paths
                        .entry(*idx)
                        .and_modify(|map| {
                            map.insert(f, step);
                        })
                        .or_default();
                    for neighbour in &self.valves.get(&f).unwrap().leads {
                        if let Some(n) = self.paths.get(idx) {
                            if !n.contains_key(neighbour) {
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

type ValveIndex = [char; 2];

#[derive(Debug, Clone)]
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

    fn idx(input: &str) -> IResult<&str, ValveIndex> {
        map(pair(anychar, anychar), |(c1, c2)| [c1, c2])(input)
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
