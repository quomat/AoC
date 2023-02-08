use std::cell::RefCell;

use nom::{
    IResult,
};

pub struct Day11;

mod Monkeys{
    pub enum ArithmeticalOperation {
        Add,
        Substract,
        Multiply,
        Divide,
    }

    pub enum Operand {
      //New, - not needed, always in a fixed position  
        Old,
        Number(u32),
    }

    pub struct Statement {
        left : Operand,
        op : ArithmeticalOperation,
        right: Operand,
      //assingnee : Operand - always New   
    }

    mod Parsers{
        use nom::{error::Error, character::complete::digit1, combinator::map};
        use nom::complete::tag;
        use nom::sequence::Tuple;
        use super::{Statement, Operand, ArithmeticalOperation};

        struct StatementParser;

        #[allow(non_snake_case)]
        fn parse_Operand(input : &str) -> nom::IResult<&str,Operand,Error<&str>> {
            alt(map(tag("old"),|_| Operand::Old),map(digit1,|num_str| u32::from_str_radix(num_str,10)))(input)
        }
        #[allow(non_snake_case)]
        fn parse_Operation(input : &str) -> nom::IResult<&str,Operand,Error<&str>> {
            let operator = char(input);
            match operator
            {
                '+' => ArithmeticalOperation::Add,
                '-' => ArithmeticalOperation::Substract,
                '*' => ArithmeticalOperation::Multiply,
                '/' => ArithmeticalOperation::Divide,
                _ => return Error {input:input, },
            }
                
        }

        #[allow(non_snake_case)]
        fn parse_Statement( input: &str) -> nom::IResult<&str, Statement, Error<&str>> {
            let new = tag("new =")(input);
            let operand1 = parse_Operand(input)?;
            let operation = parse_Operation(input)?;
            let operand2 = parse_Operand(input)?;

            Ok(Statement{operand1,operation,operand2});
        }
    }
}



pub struct Item(u32);
pub struct Test<'a> {
    truth: &'a Monkey<'a>,
    falsity: &'a Monkey<'a>,
}
pub struct Monkey<'a> {
    items: RefCell<Vec<Item>>,
    op: Operation,
    test: Test<'a>,
}
