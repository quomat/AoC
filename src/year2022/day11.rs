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

    mod Parsers{
        use std::num::ParseIntError;

        use nom::{error::Error, character::complete::{digit1, one_of}, combinator::{map,map_res}, branch::alt};
        use nom::bytes::complete::tag;
        use nom::sequence::Tuple;
        use super::{Statement, Operand, ArithmeticalOperation};

        struct StatementParser;

        #[allow(non_snake_case)]
        fn parse_Operand(input : &str) -> nom::IResult<&str,Operand> {
            alt((map(tag("old"),|_| Operand::Old),map_res(digit1,|num_str| Ok::<Operand,ParseIntError>(Operand::Number(u32::from_str_radix(num_str,10)?)))))(input)
        }
        #[allow(non_snake_case)]
        fn parse_Operation(input : &str) -> nom::IResult<&str,ArithmeticalOperation> {
            
            let (remaining,operator) = (one_of("+-*/")(input)?;
            let arithOp = match operator
            {
                '+' => ArithmeticalOperation::Add,
                '-' => ArithmeticalOperation::Substract,
                '*' => ArithmeticalOperation::Multiply,
                '/' => ArithmeticalOperation::Divide,
                _ => return Err(nom::Err::Error(nom::error::Error::new( input,  nom::error::ErrorKind::OneOf))),
            };
            Ok((remaining,arithOp))
                
        }

        #[allow(non_snake_case)]
        fn parse_Statement( input: &str) -> nom::IResult<&str, Statement, Error<&str>> {
            map(|x| (tag("new = "), parse_Operand,parse_Operation,parse_Operand).parse(x),|(new,operand1,operation,operand2)| Statement{left: operand1,op: operation,right:operand2})(input)
        }

        #[cfg(test)]
        mod tests
        {
            use super::*;

            #[test]
            fn test_parse_Statement()
            {
                let input = "new = 7 / old";
                let result = parse_Statement(input);
                

                let r = result.unwrap();
                
                assert_eq!(r.0,"");
                matches!(r.1, Statement { left : Operand::Number(7), op : ArithmeticalOperation::Divide, right : Operand::Old }); 
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
