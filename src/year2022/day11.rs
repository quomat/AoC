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
