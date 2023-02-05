use std::cell::RefCell;

pub struct Day11;

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
