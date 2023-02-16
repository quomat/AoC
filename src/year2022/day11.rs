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

    #[derive(Debug,PartialEq)] 
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

        use nom::{error::{Error, ParseError}, character::complete::{*, u32}, combinator::{map,map_res, all_consuming}, branch::alt, sequence::{delimited, preceded}, Parser, Finish, IResult, multi::{many0, separated_list0}};
        use nom::bytes::complete::tag;
        use nom::sequence::Tuple;
        use super::*;
        use nom_supreme::ParserExt;

        fn ws<'a, F, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
  where
  F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
  delimited(
    multispace0,
    inner,
    multispace0
  )
}

        fn monkey(input : &str) -> IResult<&str, Monkey<bool>> {

            let (input,(monkeyindex,items,op,test)) = (
               delimited(tag("Monkey "), u32,tag(":")),
               preceded(ws(tag("Starting items:")), 
                            separated_list0(ws(tag(",")),item)),
                preceded(ws(tag("Operation:")),
                    statement),
                preceded(ws(tag("Test:")),
                    divisible_test)
            ).parse(input)?;

            Ok((input,Monkey{ monkeyindex, items, op,test }))
        }

       
        fn operand(input : &str) -> IResult<&str,Operand> {
            let operand_parser = alt(
                (map(tag("old"),|_| Operand::Old),
                 map(u32, |i| Operand::Number(i))));
            Ok(delimited(multispace0,operand_parser,multispace0)(input)?)
        }

        fn operation(input : &str) -> IResult<&str,ArithmeticalOperation> {
            let (input,arithOp) = alt((
                map(tag("+") ,|_| ArithmeticalOperation::Add),
                map(tag("-") ,|_| ArithmeticalOperation::Substract),
                map(tag("*") ,|_| ArithmeticalOperation::Multiply),
                map(tag("/") ,|_| ArithmeticalOperation::Divide),
            ))(input)?;
            let x : Option<u32> = None;
            Ok((input,arithOp))
                
        }
            fn statement<'a>(input: &'a str) -> IResult<&'a str, Statement>
            {
                let statement_parser = map(|x| (tag("new ="), operand,operation,operand).parse(x),|(new,o1,op,o2)| Statement::new(o1,op,o2));
                    Ok(delimited(multispace0,statement_parser,multispace0)(input)?)
            }
        impl FromStr for Statement
        {
            type Err = Error<String>;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match statement.parse(s).finish()
                {
                    Ok((_remaining,output)) => Ok(output),
                    Err(Error {input, code}) => Err(Error{code,input: input.to_string()})
                }
            }
        }

        fn item(input : &str) -> IResult<&str,Item>
        {
            map(u32, Item)(input)
        }

        fn bool(input : &str) -> IResult<&str,bool>
        {
            alt((map(tag("true"),|_| true), map(tag("false"),|_| false)))(input)
        }

        fn divisible_test(input : &str) -> IResult<&str,Test<bool>>
        {
            let (input,_) = tag("divisible by ")(input)?;
            let (input, dividend) = u32(input)?;
            dbg!(dividend);
            let (input, test_map) = test_ifs(bool)(input)?;

            Ok((input,Test{ monkeyindex_if: test_map, test_fn: Box::new(move |Item(i)| i % dividend == 0)}))
        }

        
        
        fn test_record<'a, T,F>(mut key_parser : F) -> impl FnMut(&'a str) -> IResult<&'a str, (T,u32),Error<&'a str>>
        where F : FnMut(&'a str) -> IResult<&'a str, T,Error<&'a str>>
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

        fn test_ifs<'a,T,F>(mut key_parser : F) ->impl FnMut(&'a str) -> IResult<&'a str, HashMap<T,u32>>
        where F  : FnMut(&'a str) -> IResult<&'a str, T,Error<&'a str>>,
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

            #[test]
            fn parse_monkey()
            {
                let input = r#"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3"#;

                
                let result = monkey(input).finish().unwrap();
                assert_eq!(result.0,"");
                
                let monke = result.1;
                assert_eq!(monke.items,vec![Item(79),Item(98)]);
                matches!(monke.op,Statement{ left: Operand::Old , op: ArithmeticalOperation::Multiply, right: Operand::Number(19) });
                assert_eq!(monke.test.monkeyindex_if[&true],2);
                assert_eq!(monke.test.monkeyindex_if[&false],3);
                assert_eq!((monke.test.test_fn)(&Item(23)),true);
                assert_eq!((monke.test.test_fn)(&Item(24)),false);
            }
        }
    }
}




