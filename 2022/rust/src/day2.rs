use crate::day0::Day;
pub struct Day2;

impl Day<2, Vec<(Figure, Figure)>, u32> for Day2 {
    fn solve(input: Vec<(Figure, Figure)>) -> u32 {
        total(input)
    }

    fn parse(input: &str) -> Vec<(Figure, Figure)> {
        parse_by_variant(input, false)
    }

    fn parse2(input: &str) -> Vec<(Figure, Figure)> {
        parse_by_variant(input, true)
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum Figure {
    Rock,
    Paper,
    Scissors,
}

pub fn parse_enemy(s: char) -> Figure {
    match s {
        'A' => Figure::Rock,
        'B' => Figure::Paper,
        'C' => Figure::Scissors,
        _ => Figure::Rock,
    }
}

pub fn parse_own(s: char) -> Figure {
    match s {
        'X' => Figure::Rock,
        'Y' => Figure::Paper,
        'Z' => Figure::Scissors,
        _ => Figure::Rock,
    }
}

pub fn parse_own2(enemy: Figure, s: char) -> Figure {
    // TODO: Use a dictionary?
    let lose = |f| match f {
        Figure::Rock => Figure::Scissors,
        Figure::Paper => Figure::Rock,
        Figure::Scissors => Figure::Paper,
    };
    let win = |f| match f {
        Figure::Rock => Figure::Paper,
        Figure::Paper => Figure::Scissors,
        Figure::Scissors => Figure::Rock,
    };
    match s {
        'X' => lose(enemy),
        'Y' => enemy,
        _ => win(enemy),
    }
}

fn parse_by_variant(input: &str, variant: bool) -> Vec<(Figure, Figure)> {
    let v = input.split('\n');
    v.filter_map(|s| {
        let a = s.chars().next().map(parse_enemy);
        let b = a.and_then(|enemy| {
            s.chars().nth(2).map(|x| {
                if variant {
                    parse_own2(enemy, x)
                } else {
                    parse_own(x)
                }
            })
        });
        a.zip(b)
    })
    .collect()
}

pub fn total(moves: Vec<(Figure, Figure)>) -> u32 {
    moves.iter().map(|p| score(*p)).sum()
}

fn score_own(f: Figure) -> u32 {
    match f {
        Figure::Rock => 1,
        Figure::Paper => 2,
        Figure::Scissors => 3,
    }
}

fn score_round(mv: (Figure, Figure)) -> u32 {
    match mv {
        (f, g) if f == g => 3,
        (Figure::Rock, Figure::Paper) => 6,
        (Figure::Paper, Figure::Scissors) => 6,
        (Figure::Scissors, Figure::Rock) => 6,
        _ => 0,
    }
}
pub fn score(mov: (Figure, Figure)) -> u32 {
    score_round(mov) + score_own((mov).1)
}
