use std::str::FromStr;

use crate::{day0::Day, structures::tree::Node};

#[derive(Debug)]
enum ConsoleCommand {
    Ls,
    Cd(String),
    File(u64, String),
}

impl FromStr for ConsoleCommand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ConsoleCommand::*;

        let mut ws = s.split_whitespace();
        match ws.next() {
            Some("$") => match ws.next() {
                Some("cd") => {
                    if let Some(dir_name) = ws.next() {
                        Ok(Cd(String::from(dir_name)))
                    } else {
                        Err(())
                    }
                }
                Some("ls") => Ok(ConsoleCommand::Ls),
                _ => unreachable!(),
            },
            Some(number) => number
                .parse::<u64>()
                .ok()
                .and_then(|n| ws.next().map(|file_name| File(n, String::from(file_name))))
                .ok_or(()),
            None => Err(()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Entry {
    size: Option<u64>,
}

pub struct Day7 {}

impl Day<7, Node<Entry>, u64> for Day7 {
    fn solve(input: Node<Entry>) -> u64 {
        const SIZE: u64 = 100000;
        let f = input.flatten_branches();
        f.iter()
            .filter(|n| n.size.unwrap() <= SIZE)
            .map(|n| n.size.unwrap())
            .sum()
    }

    fn solve2(input: Node<Entry>) -> u64 {
        const TOTAL: u64 = 70000000;
        const NEED: u64 = 30000000;

        let unused = TOTAL - input.borrow().value.size.unwrap();

        input
            .flatten_branches()
            .iter()
            .filter(|n| n.size.unwrap() >= NEED - unused)
            .map(|n| n.size.unwrap())
            .min()
            .unwrap()
    }

    fn parse(input: &str) -> Node<Entry> {
        let mut commands = input
            .lines()
            .map(str::parse::<ConsoleCommand>)
            .filter_map(Result::ok);
        let n: Node<Entry> = Node::new(Entry { size: None });
        // let first_cmd = commands.next().unwrap();
        // dbg!(&first_cmd);

        // match first_cmd
        // {
        //     ConsoleCommand::Cd(root) => n = Rc::new(Tree::new(Entry { })),
        //     _ => unreachable!(),
        // }

        build(Node::clone(&n), &mut commands);
        tree_size(Node::clone(&n));
        n
    }
}

fn build(mut node: Node<Entry>, it: &mut impl Iterator<Item = ConsoleCommand>) {
    while let Some(cmd) = it.next() {
        dbg!(&cmd);
        match cmd {
            ConsoleCommand::Ls => (),
            ConsoleCommand::File(size, _) => {
                node.add(Entry { size: Some(size) });
            }
            ConsoleCommand::Cd(dir_name) => {
                if dir_name == ".." {
                    return;
                }
                let added = node.add(Entry { size: None });
                build(added, it);
            }
        }
    }
}

fn tree_size(node: Node<Entry>) {
    let mut gmut = node.borrow_mut();
    if gmut.value.size.is_none() {
        gmut.value.size = Some(
            gmut.children
                .iter_mut()
                .map(|c| {
                    tree_size(Node::clone(c));
                    c.borrow().value.size.unwrap()
                })
                .sum(),
        );
    }
}
