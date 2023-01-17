pub mod day0;

pub mod year2022;

#[cfg(test)]
mod tests {
    const TESTS_FILE_NAME: &str = "test.txt";
    use crate::{day0::Day, year2022::*};

    #[test]
    fn test_day1_1() {
        let result = Day1::solve_input1(TESTS_FILE_NAME);

        assert_eq!(result, 24000);
    }
    #[test]
    fn test_day1_2() {
        let result = Day1::solve_input2(TESTS_FILE_NAME);

        assert_eq!(result, 45000);
    }
    #[test]
    fn test_day2_1() {
        let result = Day2::solve_input1(TESTS_FILE_NAME);

        assert_eq!(result, 15);
    }
    #[test]
    fn test_day2_2() {
        let result = Day2::solve_input2(TESTS_FILE_NAME);

        assert_eq!(result, 12);
    }
    #[test]
    fn test_day3_1() {
        let result = Day3::solve_input1(TESTS_FILE_NAME);

        assert_eq!(result, 157);
    }
    #[test]
    fn test_day3_2() {
        let result = Day3::solve_input2(TESTS_FILE_NAME);

        assert_eq!(result, 70);
    }
    #[test]
    fn test_day4_1() {
        let result = Day4::solve_input1(TESTS_FILE_NAME);

        assert_eq!(result, 2);
    }
    #[test]
    fn test_day4_2() {
        let result = Day4::solve_input2(TESTS_FILE_NAME);

        assert_eq!(result, 4);
    }
}
