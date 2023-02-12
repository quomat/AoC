use std::cell::RefCell;

use self::Monkeys::Statement;



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

    impl Statement
    {
        fn new(operand1 : Operand, operation : ArithmeticalOperation, operand2 : Operand) -> Statement
        {
            Statement{left: operand1,op: operation,right:operand2}
        }
    }

    mod parsers{
        use std::str::FromStr;

        use nom::{error::Error, character::complete::*, combinator::{map,map_res}, branch::alt, sequence::delimited, Parser, Finish};
        use nom::bytes::complete::tag;
        use nom::sequence::Tuple;
        use super::{Statement, Operand, ArithmeticalOperation};

        
        #[allow(non_snake_case)]
        fn parse_Operand(input : &str) -> nom::IResult<&str,Operand> {
            let operand_parser = alt((map(tag("old"),|_| Operand::Old),map(nom::character::complete::u32, |i| Operand::Number(i))));
            Ok(delimited(multispace0,operand_parser,multispace0)(input)?)
        }
        #[allow(non_snake_case)]
        fn parse_Operation(input : &str) -> nom::IResult<&str,ArithmeticalOperation> {
            let (input,arithOp) = alt((
                map(tag("+") ,|_| ArithmeticalOperation::Add),
                map(tag("-") ,|_| ArithmeticalOperation::Substract),
                map(tag("*") ,|_| ArithmeticalOperation::Multiply),
                map(tag("/") ,|_| ArithmeticalOperation::Divide),
            ))(input)?;
            let x : Option<u32> = None;
            Ok((input,arithOp))
                
        }
        struct StatementParser;
        impl<'a> Parser<&'a str,Statement, Error<&'a str>> for StatementParser
        {
            fn parse(&mut self, input: &'a str) -> nom::IResult<&'a str, Statement>
            {
                let statement_parser = map(|x| (tag("new ="), parse_Operand,parse_Operation,parse_Operand).parse(x),|(new,o1,op,o2)| Statement::new(o1,op,o2));
                    Ok(delimited(multispace0,statement_parser,multispace0)(input)?)
            }
        }
        impl FromStr for Statement
        {
            type Err = Error<String>;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let mut parser = StatementParser{};
                match parser.parse(s).finish()
                {
                    Ok((_remaining,output)) => Ok(output),
                    Err(Error {input, code}) => Err(Error{code,input: input.to_string()})
                }
            }
        }

        #[cfg(test)]
        mod tests
        {
            use super::*;

            #[test]
            fn parse_statement()
            {
                let input = "new = 7 / old";
                
                let result = input.parse::<Statement>().unwrap();
                
                matches!(result, Statement { left : Operand::Number(7), op : ArithmeticalOperation::Divide, right : Operand::Old }); 
            }
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
    op: Statement,
    test: Test<'a>,
}
