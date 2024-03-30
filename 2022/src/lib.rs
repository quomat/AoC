pub mod day0;

pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

pub mod structures;
pub mod utils;

#[cfg(test)]
mod tests {
    const TESTS_FILE_NAME: &str = "test";
    const TESTS_ADDITIONAL_FILE_NAME: &str = "test2";
    use crate::*;
    use crate::day0::Day;

    macro_rules! day {
        ($n:tt, $p:tt, $answer:expr) => {
            paste::item! {
                #[test]
                fn [<day $n _ $p>]()
                {
                    let result = [<day $n>]::[<Day $n>]::[<solve_input $p>](TESTS_FILE_NAME);

                    assert_eq!(result,$answer);
                }
            }
        };
    }

    day!(1, 1, 24000);
    day!(1, 2, 45000);
    day!(2, 1, 15);
    day!(2, 2, 12);
    day!(3, 1, 157);
    day!(3, 2, 70);
    day!(4, 1, 2);
    day!(4, 2, 4);

    day!(5, 1, "CMZ");
    day!(5, 2, "MCD");
    day!(6, 1, 7);
    day!(6, 2, 19);
    day!(7, 1, 95437);
    day!(7, 2, 24933642);
    day!(8, 1, 21);
    day!(8, 2, 8);
    day!(9, 1, 13);
    day!(9, 2, 1);

    #[test]
    fn day9_3() {
        let result = day9::Day9::solve_input2(TESTS_ADDITIONAL_FILE_NAME);
        assert_eq!(result, 36);
    }

    day!(10, 1, day10::ComputerOutput::SignalSum(13140));

    #[test]
    fn day10_2() {
        let result = day10::Day10::solve_input2(TESTS_FILE_NAME);

        let crt = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
            .to_owned();

        assert_eq!(result, day10::ComputerOutput::Screen(crt));
    }

    day!(11, 1, 10605);

    day!(11, 2, 2713310158);

    day!(12, 1, 31);

    day!(12, 2, 29);

    day!(13, 1, vec![1, 2, 4, 6]);
    day!(13, 2, vec![10, 14]);

    day!(14, 1, 24);
    day!(14, 2, 93);
    #[test]
    fn day15_1() {
        let result = day15::Day15::<10>::solve_input1(TESTS_FILE_NAME);
        assert_eq!(result, 26);
    }
    #[test]
    fn day15_2() {
        let result = day15::Day15::<10>::solve_input2(TESTS_FILE_NAME);
        assert_eq!(result, 56000011);
    }

    #[test]
    fn day16_1() {
        let result = day16::Day16::<30>::solve_input1(TESTS_FILE_NAME);
        assert_eq!(result, 1651)
    }

    #[test]
    fn day16_2() {
        let result = day16::Day16::<26>::solve_input2(TESTS_FILE_NAME);
        assert_eq!(result, 1707)
    }

    day!(17, 1, 3068);
    day!(17, 2, 1514285714288);

    #[test]
    fn day18_1_easy() {
        assert_eq!(day18::Day18::solve(day18::Day18::parse("1,1,1\n1,1,2,\n1,2,1")), 14);
    }
    day!(18, 1, 64);
    day!(18, 2, 58);

    #[test]
    fn day19_1() {
        assert_eq!(day19::Day19::<24>::solve_input1(TESTS_FILE_NAME), 9);
    }
}
