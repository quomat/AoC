use nom::{
    IResult,
};

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


fn parser_Statement(input : &str) -> IResult<&str,
{
    
}

