use crate::day0::Day;

pub struct Day8 {}

impl Day<2022, 8, Vec<Vec<u32>>, usize> for Day8 {
    fn solve(input: Vec<Vec<u32>>) -> usize {
        let n = input.len();
        let mut visible: Vec<Vec<bool>> = vec![vec![false; n]; n];

        scavenge(|i, j| input[i][j], |i, j| visible[i][j] = true, n);
        scavenge(|i, j| input[j][i], |i, j| visible[j][i] = true, n);

        4 * (n - 1)
            + visible
                .into_iter()
                .map(|v| v.into_iter().filter(|&x| x).count())
                .sum::<usize>()
    }

    fn solve2(input: Vec<Vec<u32>>) -> usize {
        let n = input.len();
        // brute force :(
        for i in 0..n {
            for j in 0..n {
                dbg!(i, j, scenic_score(&input, i, j));
            }
        }
        (1..n - 1)
            .map(|k| {
                (1..n - 1)
                    .map(|m| scenic_score(&input, k, m))
                    .max()
                    .unwrap()
            })
            .max()
            .unwrap()
    }

    fn parse(input: &str) -> Vec<Vec<u32>> {
        input
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect()
    }
}
type Ray = Box<dyn Fn(i32, i32) -> (i32, i32)>;
fn scenic_score(forest: &Vec<Vec<u32>>, i: usize, j: usize) -> usize {
    let rays: Vec<Ray> = vec![
        Box::new(move |i, j| (i - 1, j)),
        Box::new(move |i, j| (i + 1, j)),
        Box::new(move |i, j| (i, j - 1)),
        Box::new(move |i, j| (i, j + 1)),
    ];
    rays.iter().map(|f| raycast(f, forest, i, j)).product()
}

fn raycast<F>(f: F, forest: &Vec<Vec<u32>>, i0: usize, j0: usize) -> usize
where
    F: Fn(i32, i32) -> (i32, i32),
{
    println!("[raycast] start, for tree <<{0},{1}>>", i0, j0);
    let n = forest.len();
    let mut s = 0;
    let mut i: i32 = i0 as i32;
    let mut j: i32 = j0 as i32;
    let max = forest[i0][j0];
    loop {
        (i, j) = f(i, j);
        if !(i >= 0 && i < n as i32 && j >= 0 && j < n as i32) {
            println!("[raycast] out of bounds, for tree <<{0},{1}>>", i0, j0);
            break;
        }

        s += 1;
        println!("[raycast] still inbound for tree <<{0},{1}>>, currently at <<{2},{3}>> with {4} tree seen",i0,j0,i,j,s);
        if max <= forest[i as usize][j as usize] {
            println!("[raycast] tree at <<{2},{3}>> is too high, height: <<{4}>> while tree <<{0},{1}>> has only height {5}",i0,j0,i,j,forest[i as usize][j as usize], max);
            break;
        }
    }
    println!("[raycast] finished. , seen {0} trees", s);
    s
}

fn scavenge<F, G>(input: F, mut visibility: G, n: usize)
where
    F: Fn(usize, usize) -> u32,
    G: FnMut(usize, usize),
{
    for i in 1..n - 1 {
        let mut l = 1;
        let mut r = n - 2;
        let mut l_max = input(i, 0);
        let mut r_max = input(i, n - 1);
        while r >= l {
            if l_max < r_max {
                if input(i, l) > l_max {
                    visibility(i, l);
                    l_max = input(i, l);
                }
                l += 1;
            } else {
                if input(i, r) > r_max {
                    visibility(i, r);
                    r_max = input(i, r);
                }
                r -= 1;
            }
        }
    }
}
