pub mod day0;

pub mod year2022;

pub mod structures;

#[cfg(test)]
mod tests {
    const TESTS_FILE_NAME: &str = "test";
    const TESTS_ADDITIONAL_FILE_NAME: &str = "test2";
    use crate::{day0::Day, year2022::*};

    macro_rules! day {
        ($n:tt, $p:tt, $answer:expr) => {
            paste::item! {
                #[test]
                fn [<test_day $n _ $p>]()
                {
                    let result = [<Day $n>]::[<solve_input $p>](TESTS_FILE_NAME);

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
    fn test_day9_3() {
        let result = Day9::solve_input2(TESTS_ADDITIONAL_FILE_NAME);
        assert_eq!(result, 36);
    }

    day!(10, 1, day10::ComputerOutput::SignalSum(13140));

    #[test]
    fn test_day10_2() {
        let result = Day10::solve_input2(TESTS_FILE_NAME);

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
    fn test_day15_1() {
        let result = Day15::<10>::solve_input1(TESTS_FILE_NAME);
        assert_eq!(result, 26);
    }
    #[test]
    fn test_day15_2() {
        let result = Day15::<10>::solve_input2(TESTS_FILE_NAME);
        assert_eq!(result, 56000011);
    }
}
