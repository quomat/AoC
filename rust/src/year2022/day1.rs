use crate::day0::Day;

pub struct Day1;

impl Day<2022, 1, Vec<Vec<u32>>, u32> for Day1 {
    fn parse(input: &str) -> Vec<Vec<u32>> {
        input
            .split("\n\n")
            .map(|group| group.lines().map(|x| x.parse::<u32>().unwrap()).collect())
            .collect()
    }

    fn solve(input: Vec<Vec<u32>>) -> u32 {
        sum_max(input)
    }

    fn solve2(input: Vec<Vec<u32>>) -> u32 {
        sum_max_3(input)
    }
}

fn sum_max(cals: Vec<Vec<u32>>) -> u32 {
    let mut max: u32 = 0;
    for v in cals {
        let s = sum(v);
        if s > max {
            max = s;
        }
    }

    max
}

fn sum_max_3(cals: Vec<Vec<u32>>) -> u32 {
    let mut max1 = 0;
    let mut max2 = 0;
    let mut max3 = 0;
    for v in cals {
        let s = sum(v);
        if s > max1 {
            max3 = max2;
            max2 = max1;
            max1 = s;
        } else if s > max2 {
            max3 = max2;
            max2 = s;
        } else if s > max3 {
            max3 = s;
        }
    }
    max1 + max2 + max3
}

fn sum(c: Vec<u32>) -> u32 {
    let mut sum: u32 = 0;
    for x in c {
        sum += x;
    }
    sum
}
