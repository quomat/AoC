use crate::day0::*;

pub struct Day6 {}

struct AsciiMultiSet {
    letters: [u8; 26],
}

impl AsciiMultiSet {
    fn new() -> AsciiMultiSet {
        AsciiMultiSet { letters: [0; 26] }
    }

    fn insert(&mut self, c: char) {
        let b: usize = c as usize;
        self.letters[b - b'a' as usize] += 1;
    }

    fn remove(&mut self, c: char) {
        let b: usize = c as usize;
        self.letters[b - b'a' as usize] = self.letters[b - b'a' as usize].saturating_sub(1);
    }

    fn len(&self) -> usize {
        self.letters.into_iter().filter(|x| *x > 0).count()
    }
}

impl Day<2022, 6, Vec<char>, usize> for Day6 {
    fn solve(input: Vec<char>) -> usize {
        solve_for(input, 4)
    }

    fn solve2(input: Vec<char>) -> usize {
        solve_for(input, 14)
    }

    fn parse(input: &str) -> Vec<char> {
        input.chars().collect()
    }
}

fn solve_for(input: Vec<char>, p: usize) -> usize {
    let mut set: AsciiMultiSet = AsciiMultiSet::new();

    for i in 0..input.len() {
        if set.len() == p {
            return i;
        }
        if i >= p {
            set.remove(input[i - p]);
        }
        set.insert(input[i]);
    }
    unreachable!()
}
