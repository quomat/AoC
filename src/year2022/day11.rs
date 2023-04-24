use crate::day0::Day;

use self::monkeys::*;

pub struct Day11;

impl Day<2022, 11, Vec<Monkey<bool>>, u64> for Day11 {
    fn solve(input: Vec<Monkey<bool>>) -> u64 {
        let n = 20;

        let mut arena = MonkeyArena::new(input, true);

        for _i in 0..n {
            arena.round();
            // println!("After round {0}, the monkeys are holding items with these worry levels:",_i);
        }

        arena.get_monkey_business()
    }

    fn solve2(input: Vec<Monkey<bool>>) -> u64 {
        let n = 10000;

        let mut arena = MonkeyArena::new(input, false);

        for _i in 0..n {
            arena.round();
            // // println!("After round {0}, the monkeys are holding items with these worry levels:",_i);
            // arena._print();
        }
        // arena._print_monkey_business();
        arena.get_monkey_business()
    }

    fn parse(input: &str) -> Vec<Monkey<bool>> {
        monkeys::parsers::monkeys(input).unwrap().1
    }
}

mod monkeys {
    use std::collections::{HashMap, VecDeque};
    use std::fmt::Debug;
    use std::hash::Hash;

    #[derive(Debug, Clone, Copy)]
    pub enum ArithmeticalOperation {
        Add,
        Substract,
        Multiply,
        Divide,
    }

    trait Arithmeticable: Clone {
        fn from_number(s: u64) -> Self;

        fn compute(op: ArithmeticalOperation, x: Self, y: Self) -> Self;
    }

    impl Clone for Item {
        fn clone(&self) -> Self {
            Item(self.0)
        }
    }

    impl Arithmeticable for Item {
        fn from_number(s: u64) -> Self {
            Item(s)
        }

        fn compute(op: ArithmeticalOperation, Item(x): Self, Item(y): Self) -> Self {
            match op {
                ArithmeticalOperation::Add => Item(x + y),
                ArithmeticalOperation::Substract => Item(x - y),
                ArithmeticalOperation::Multiply => Item(x * y),
                ArithmeticalOperation::Divide => Item(x / y),
            }
        }
    }

    #[derive(Debug)]
    pub enum Operand {
        //New, - not needed, always in a fixed position
        Old,
        Number(u64),
    }

    impl Operand {
        fn get_arith<T>(&self, old: &T) -> T
        where
            T: Arithmeticable + Clone,
        {
            match self {
                Operand::Old => old.clone(),
                Operand::Number(i) => Arithmeticable::from_number(*i),
            }
        }
    }

    #[derive(Debug)]
    pub struct Statement {
        left: Operand,
        op: ArithmeticalOperation,
        right: Operand,
        //assingnee : Operand - always New
    }

    #[derive(Debug, PartialEq)]
    pub struct Item(u64);
    impl Item {
        fn devalue(&mut self, rem: Option<u32>) {
            if let Some(r) = rem {
                self.0 %= r as u64;
            } else {
                self.0 /= 3;
            }
        }
    }

    pub struct Test<T>
    where
        T: Eq + Hash,
    {
        monkeyindex_if: HashMap<T, u32>,
        test_fn: Box<dyn Fn(&Item) -> T>,
        dividend: u32,
    }
    impl<T> Debug for Test<T>
    where
        T: Eq + Hash + Debug,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "Hi, Test here. Here is my monkeyindex_if: {0:?} \n and \n test_fn({1:?}) == {2:?}",
                self.monkeyindex_if,
                Item(0),
                (self.test_fn)(&Item(0))
            )
        }
    }

    impl<T> Test<T>
    where
        T: Eq + Hash,
    {
        fn run(&self, item: &Item) -> u32 {
            *self.monkeyindex_if.get(&(self.test_fn)(item)).unwrap()
        }
    }

    pub struct MonkeyArena<T>
    where
        T: Eq + Hash,
    {
        arena: HashMap<u32, Monkey<T>>,
        tally: HashMap<u32, u64>,
        rem: Option<u32>,
    }

    impl<T> MonkeyArena<T>
    where
        T: Eq + Hash,
    {
        pub fn new(monkeys: Vec<Monkey<T>>, manageable: bool) -> MonkeyArena<T> {
            let mut monkey_arena = MonkeyArena {
                arena: HashMap::new(),
                tally: HashMap::new(),
                rem: if manageable {
                    None
                } else {
                    Some(monkeys.iter().map(|m| m.test.dividend).product())
                },
            };
            for monkey in monkeys {
                let index = monkey.monkeyindex;
                monkey_arena.arena.insert(index, monkey);
                monkey_arena.tally.insert(index, 0);
            }

            monkey_arena
        }

        fn act(&mut self, monkey_index: &u32) {
            // println!("Monkey {}:",monkey_index);
            loop {
                let item: Item;
                let target: u32;
                {
                    let monke = &mut self.arena.get_mut(monkey_index).unwrap();
                    if let Some(r) = monke.throw(self.rem) {
                        item = r.0;
                        target = r.1;
                    } else {
                        break;
                    }
                }

                *self.tally.get_mut(monkey_index).unwrap() += 1;
                let target_monke = &mut self.arena.get_mut(&target).unwrap();
                //     println!("\t\tItem with worry level {0} is thrown to monkey {1}.",item.clone().0,target);
                target_monke.items.push_back(item);
            }
        }

        pub(crate) fn round(&mut self) {
            let mut keys: Vec<u32> = self.arena.keys().copied().collect();
            keys.sort();
            for index in keys {
                //     dbg!(index);
                self.act(&index);
            }
        }

        pub(crate) fn get_monkey_business(self) -> u64 {
            let mut tally = self.tally.into_values().collect::<Vec<u64>>();
            tally.sort();
            tally.into_iter().rev().take(2).product()
        }

        pub(crate) fn _print(&self) {
            for _i in self.arena.keys() {
                println!("Monkey {0}: {1:?}", _i, self.arena[_i].items);
            }
        }

        pub(crate) fn _print_monkey_business(&self) {
            for i in self.arena.keys() {
                println!("Monkey {0} inspected items {1} times.", i, self.tally[i])
            }
        }
    }

    #[derive(Debug)]
    pub struct Monkey<T>
    where
        T: Eq + Hash,
    {
        monkeyindex: u32,
        items: VecDeque<Item>,
        op: Statement,
        test: Test<T>,
    }

    impl<T> Monkey<T>
    where
        T: Eq + Hash,
    {
        fn inspect(&mut self, rem: Option<u32>) {
            if let Some(it) = self.items.get_mut(0) {
                //     println!("\tMonkey inspects an item with a worry level of {}.",it.0);
                let _oldd = it.clone();
                self.op.execute(it);
                //     println!("\t\tWorry level is changed from {0} to {1}",oldd.0,it.0);
                it.devalue(rem);
                //     println!("\t\tMonkey gets bored with item. Worry level is divided by 3 to {}",it.0);
            }
        }

        fn throw(&mut self, rem: Option<u32>) -> Option<(Item, u32)>
        where
            T: Eq + Hash,
        {
            self.inspect(rem);
            let item = self.items.pop_front()?;
            let index = self.test.run(&item);

            Some((item, index))
        }
    }

    impl Statement {
        fn new(
            operand1: Operand,
            operation: ArithmeticalOperation,
            operand2: Operand,
        ) -> Statement {
            Statement {
                left: operand1,
                op: operation,
                right: operand2,
            }
        }

        fn execute<T>(&self, old: &mut T)
        where
            T: Arithmeticable,
        {
            let left = self.left.get_arith(old);
            let right = self.right.get_arith(old);
            *old = <T>::compute(self.op, left, right);
        }
    }

    pub(crate) mod parsers {
        use std::{fmt::Debug, str::FromStr};

        use super::*;
        use nom::{
            branch::alt,
            bytes::complete::tag,
            character::complete::{u32, *},
            combinator::{all_consuming, map},
            error::*,
            multi::{many0, separated_list0},
            sequence::Tuple,
            sequence::{delimited, preceded},
            Finish, IResult, Parser,
        };

        fn ws<'a, F, O, E: ParseError<&'a str>>(
            inner: F,
        ) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
        where
            F: FnMut(&'a str) -> IResult<&'a str, O, E>,
        {
            delimited(multispace0, inner, multispace0)
        }

        fn monkey(input: &str) -> IResult<&str, Monkey<bool>> {
            let (input, (monkeyindex, items, op, test)) = (
                delimited(tag("Monkey "), u32, tag(":")),
                preceded(
                    ws(tag("Starting items:")),
                    separated_list0(ws(tag(",")), item),
                ),
                preceded(ws(tag("Operation:")), statement),
                preceded(ws(tag("Test:")), divisible_test),
            )
                .parse(input)?;

            Ok((
                input,
                Monkey {
                    monkeyindex,
                    items: items.into_iter().collect(),
                    op,
                    test,
                },
            ))
        }

        pub(crate) fn monkeys(input: &str) -> IResult<&str, Vec<Monkey<bool>>> {
            all_consuming(many0(ws(monkey)))(input)
        }

        fn operand(input: &str) -> IResult<&str, Operand> {
            let operand_parser = alt((
                map(tag("old"), |_| Operand::Old),
                map(u64, Operand::Number),
            ));
            delimited(multispace0, operand_parser, multispace0)(input)
        }

        fn operation(input: &str) -> IResult<&str, ArithmeticalOperation> {
            let (input, arith_op) = alt((
                map(tag("+"), |_| ArithmeticalOperation::Add),
                map(tag("-"), |_| ArithmeticalOperation::Substract),
                map(tag("*"), |_| ArithmeticalOperation::Multiply),
                map(tag("/"), |_| ArithmeticalOperation::Divide),
            ))(input)?;
            Ok((input, arith_op))
        }
        fn statement(input: &str) -> IResult<&str, Statement> {
            let statement_parser = map(
                |x| (tag("new ="), operand, operation, operand).parse(x),
                |(_, o1, op, o2)| Statement::new(o1, op, o2),
            );
            delimited(multispace0, statement_parser, multispace0)(
                input,
            )
        }
        impl FromStr for Statement {
            type Err = Error<String>;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match statement.parse(s).finish() {
                    Ok((_remaining, output)) => Ok(output),
                    Err(Error { input, code }) => Err(Error {
                        code,
                        input: input.to_string(),
                    }),
                }
            }
        }

        fn item(input: &str) -> IResult<&str, Item> {
            map(u64, Item)(input)
        }

        fn bool(input: &str) -> IResult<&str, bool> {
            alt((map(tag("true"), |_| true), map(tag("false"), |_| false)))(input)
        }

        fn divisible_test(input: &str) -> IResult<&str, Test<bool>> {
            let (input, _) = tag("divisible by ")(input)?;
            let (input, dividend) = u32(input)?;
            // dbg!(dividend);
            let (input, test_map) = test_ifs(bool)(input)?;

            Ok((
                input,
                Test {
                    monkeyindex_if: test_map,
                    dividend,
                    test_fn: Box::new(move |Item(i)| i % (dividend as u64) == 0),
                },
            ))
        }

        fn test_record<'a, T, F>(
            mut key_parser: F,
        ) -> impl FnMut(&'a str) -> IResult<&'a str, (T, u32), Error<&'a str>>
        where
            F: FnMut(&'a str) -> IResult<&'a str, T, Error<&'a str>>,
        {
            move |input| {
                //     dbg!(input);
                let (input, _) = tag("If")(input)?;
                let (input, key) = delimited(multispace0, &mut key_parser, tag(": "))(input)?;
                let (input, _) = tag("throw to monkey ")(input)?;
                let (input, idx) = u32(input)?;
                //     dbg!(idx);
                Ok((input, (key, idx)))
            }
        }

        fn test_ifs<'a, T, F>(
            mut key_parser: F,
        ) -> impl FnMut(&'a str) -> IResult<&'a str, HashMap<T, u32>>
        where
            F: FnMut(&'a str) -> IResult<&'a str, T, Error<&'a str>>,
            T: Hash + Eq + Debug,
        {
            move |input| {
                let mut result = HashMap::new();
                let mut running_input = input;
                while let Ok((remining_input, record)) =
                    delimited(multispace0, test_record(&mut key_parser), multispace0)(running_input)
                {
                    running_input = remining_input;
                    result.insert(record.0, record.1);
                }
                //     dbg!(&result);
                Ok((running_input, result))
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn parse_statement() {
                let input = "new = 7 / old";

                let result = input.parse::<Statement>().unwrap();

                matches!(
                    result,
                    Statement {
                        left: Operand::Number(7),
                        op: ArithmeticalOperation::Divide,
                        right: Operand::Old
                    }
                );
            }

            #[test]
            fn parse_test_record() {
                let input = "If true: throw to monkey 2";

                let result = test_record(bool)(input).finish().unwrap();

                assert_eq!(result.0, "");
                assert_eq!(result.1, (true, 2));
            }

            #[test]
            fn parse_test() {
                let input = r#"divisible by 23
                    If true: throw to monkey 2
                    If false: throw to monkey 3
                "#;
                //tricky indentation should work

                let result = divisible_test(input).finish().unwrap();

                assert_eq!(result.0, "");

                assert_eq!(result.1.monkeyindex_if[&true], 2);
                assert_eq!(result.1.monkeyindex_if[&false], 3);
                assert!((result.1.test_fn)(&Item(23)));
                assert!(!(result.1.test_fn)(&Item(24)));
            }

            #[test]
            fn parse_monkey() {
                let input = r#"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3"#;

                let result = monkey(input).finish().unwrap();
                assert_eq!(result.0, "");

                let monke = result.1;
                assert_eq!(monke.items, vec![Item(79), Item(98)]);
                matches!(
                    monke.op,
                    Statement {
                        left: Operand::Old,
                        op: ArithmeticalOperation::Multiply,
                        right: Operand::Number(19)
                    }
                );
                assert_eq!(monke.test.monkeyindex_if[&true], 2);
                assert_eq!(monke.test.monkeyindex_if[&false], 3);
                assert!((monke.test.test_fn)(&Item(23)));
                assert!(!(monke.test.test_fn)(&Item(24)));
            }
        }
    }
}
