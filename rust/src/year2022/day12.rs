use crate::day0::Day;

pub struct HeightMap {
    map: Vec<Vec<u8>>,
    start: (usize, usize),
    end: (usize, usize),
}

pub struct Day12;

impl Day<2022, 12, HeightMap, u32> for Day12 {
    fn solve(input: HeightMap) -> u32 {
        let n = input.map.len();
        let m = input.map[0].len();
        // dbg!(&input.map);
        let mut steps: u32 = 0;
        let mut opens = vec![input.start];
        let mut visited: Vec<Vec<Option<u32>>> = vec![vec![None; m]; n];
        loop {
            let incoming = opens.clone();
            if incoming.is_empty() {
                return u32::MAX;
            }
            opens.clear();
            // dbg!(steps);
            for open in incoming {
                if open == input.end {
                    return steps;
                }
                if visited[open.0][open.1].is_some() {
                    continue;
                } else {
                    visited[open.0][open.1] = Some(steps);
                }
                if open.0 > 0
                    && input.map[open.0][open.1] >= input.map[open.0 - 1][open.1].saturating_sub(1)
                {
                    opens.push((open.0 - 1, open.1));
                }
                if open.0 < n - 1
                    && input.map[open.0][open.1] >= input.map[open.0 + 1][open.1].saturating_sub(1)
                {
                    opens.push((open.0 + 1, open.1));
                }
                if open.1 > 0
                    && input.map[open.0][open.1] >= input.map[open.0][open.1 - 1].saturating_sub(1)
                {
                    opens.push((open.0, open.1 - 1));
                }
                if open.1 < m - 1
                    && (input.map[open.0][open.1]
                        >= input.map[open.0][open.1 + 1].saturating_sub(1))
                {
                    opens.push((open.0, open.1 + 1));
                }
            }
            steps += 1;
        }
    }

    fn solve2(input: HeightMap) -> u32 {
        let starting_pos: Vec<(usize, usize)> = input
            .map
            .clone()
            .into_iter()
            .enumerate()
            .flat_map(move |(i, b)| {
                b.into_iter()
                    .enumerate()
                    .filter_map(move |(j, b)| if b == b'a' { Some((i, j)) } else { None })
            })
            .collect();

        starting_pos
            .into_iter()
            .map(|s| {
                Day12::solve(HeightMap {
                    map: input.map.clone(),
                    start: s,
                    end: input.end,
                })
            })
            .min()
            .unwrap()
    }
    fn parse(input: &str) -> HeightMap {
        let mut start: Option<(usize, usize)> = None;
        let mut end: Option<(usize, usize)> = None;
        let chars = input
            .lines()
            .enumerate()
            .map(|(i, l)| {
                l.bytes()
                    .enumerate()
                    .map(|(j, b)| match b {
                        b'S' => {
                            start = Some((i, j));
                            b'a'
                        }
                        b'E' => {
                            end = Some((i, j));
                            b'z'
                        }
                        _ => b,
                    })
                    .collect()
            })
            .collect();
        HeightMap {
            map: chars,
            start: start.unwrap(),
            end: end.unwrap(),
        }
    }
}
