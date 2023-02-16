use std::cell::RefCell;

use self::Monkeys::Statement;



pub struct Day11;

mod Monkeys{
    use std::collections::HashMap;
    use std::hash::Hash;
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

     
    pub struct Item(u32);
    pub struct Test<T>
    where T : Eq + Hash,
     {
        monkeyindex_if: HashMap<T,u32>,
        test_fn : Box<dyn Fn(&Item) -> T> 
    }
    impl<T> Test<T>
    where T : Eq + Hash
    {
        fn run(&self, item : &Item) -> u32
        {
            *self.monkeyindex_if.get(&(self.test_fn)(item)).unwrap()
        }
    }
    
    pub struct Monkey<T> 
    where T : Eq + Hash
    {
        monkeyindex : u32,
        items: Vec<Item>,
        op: Statement,
        test: Test<T>,
    }

    impl Statement
    {
        fn new(operand1 : Operand, operation : ArithmeticalOperation, operand2 : Operand) -> Statement
        {
            Statement{left: operand1,op: operation,right:operand2}
        }
    }

    mod parsers{
        use std::{str::FromStr, fmt::Debug};

        use nom::{error::Error, character::complete::*, combinator::{map,map_res, all_consuming}, branch::alt, sequence::delimited, Parser, Finish};
        use nom::bytes::complete::tag;
        use nom::sequence::Tuple;
        use super::*;
        use nom_supreme::ParserExt;
        
        fn operand(input : &str) -> nom::IResult<&str,Operand> {
            let operand_parser = alt(
                (map(tag("old"),|_| Operand::Old),
                 map(nom::character::complete::u32, |i| Operand::Number(i))));
            Ok(delimited(multispace0,operand_parser,multispace0)(input)?)
        }

        fn operation(input : &str) -> nom::IResult<&str,ArithmeticalOperation> {
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
                let statement_parser = map(|x| (tag("new ="), operand,operation,operand).parse(x),|(new,o1,op,o2)| Statement::new(o1,op,o2));
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

        fn item(input : &str) -> nom::IResult<&str,Item>
        {
            map(nom::character::complete::u32, Item)(input)
        }

        fn bool(input : &str) -> nom::IResult<&str,bool>
        {
            alt((map(tag("true"),|_| true), map(tag("false"),|_| false)))(input)
        }

        fn divisible_test(input : &str) -> nom::IResult<&str,Test<bool>>
        {
            let (input,_) = tag("divisible by ")(input)?;
            let (input, dividend) = u32(input)?;
            dbg!(dividend);
            let (input, test_map) = test_ifs(bool)(input)?;

            Ok((input,Test{ monkeyindex_if: test_map, test_fn: Box::new(move |Item(i)| i % dividend == 0)}))
        }

        
        
        fn test_record<'a, T,F>(mut key_parser : F) -> impl FnMut(&'a str) -> nom::IResult<&'a str, (T,u32),Error<&'a str>>
        where F : FnMut(&'a str) -> nom::IResult<&'a str, T,Error<&'a str>>
        {
            move |input|{
                dbg!(input);
                let (input, _) = tag("If")(input)?;
                let (input, key) = delimited(multispace0, &mut key_parser, tag(": "))(input)?;
                let (input, _) = tag("throw to monkey ")(input)?; 
                let (input, idx) = u32(input)?;
                dbg!(idx);
                Ok((input,(key,idx)))
            }
        }

        fn test_ifs<'a,T,F>(mut key_parser : F) ->impl FnMut(&'a str) -> nom::IResult<&'a str, HashMap<T,u32>>
        where F  : FnMut(&'a str) -> nom::IResult<&'a str, T,Error<&'a str>>,
         T  : Hash + Eq + Debug
        {
            move |input|
            {
                let mut result = HashMap::new();
                let mut running_input = input;
                while let Ok((remining_input,record)) = delimited(multispace0, test_record(&mut key_parser), multispace0)(running_input)
                {
                        running_input = remining_input;
                        result.insert(record.0,record.1);
                }
                dbg!(&result);
                Ok((running_input,result))
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

            #[test]
            fn parse_test_record()
            {
                let input = "If true: throw to monkey 2";

                let result = test_record(bool)(input).finish().unwrap();

                assert_eq!(result.0, "");
                assert_eq!(result.1, (true,2));
            }

            #[test]
            fn parse_test()
            {
                let input = r#"divisible by 23
                    If true: throw to monkey 2
                    If false: throw to monkey 3
                "#;
                //tricky indentation should work

                let result = divisible_test(input).finish().unwrap();

                assert_eq!(result.0,"");

                assert_eq!(result.1.monkeyindex_if[&true], 2);
                assert_eq!(result.1.monkeyindex_if[&false], 3);
                assert_eq!((result.1.test_fn)(&Item(23)),true);
                assert_eq!((result.1.test_fn)(&Item(24)),false);
                
            }
        }
    }
}




