use aoc::structures::tree::*;

fn main()
{
    let t : Tree<u16> = Tree::new(5);
    let mut node = Node::from(t);
    let mut node1 = node.add(8);
    node1.add(11);
    node1.add(432);
    let ft = find_node(Node::clone(&node),&8);
    let ft2 = find_node(Node::clone(&node), &11);
    println!("{:?}",node);
    println!("{:?}",ft);
    println!("{:?}",ft2);

}