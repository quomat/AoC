use std::{cell::{RefCell}, rc::{Rc}, ops::Deref};

#[derive(Debug)]
pub struct Node<T>(pub Rc<RefCell<Tree<T>>>);

impl<T> Node<T>
{
    pub fn new(val : T) -> Node<T>
    {
        Node(Rc::new(RefCell::new(Tree::new(val))))
    }

    pub fn from(val : Tree<T>) -> Node<T>
    {
        Node(Rc::new(RefCell::new(val)))
    }

    pub fn add(&mut self, val : T) -> Node<T>
    {
        let n = Node::new(val);
        self.0.borrow_mut().children.push(Node::clone(&n));
        n
    }

    pub fn flatten(&self) -> Vec<T>
    where T : Clone
    {
        let mut result = Vec::new();
        let tree = self.borrow();
        for child_node in tree.children.iter()
        {
            result.append(&mut child_node.flatten());
        }
        result.push(tree.value.clone());
        result
    }

    pub fn is_leaf(&self) -> bool
    {
        dbg!("[is_leaf}, checking...",self.borrow().children.len());
        self.borrow().children.len() == 0
    }

    pub fn flatten_branches(&self) -> Vec<T>
    where T : Clone
    {
        let mut result = Vec::new();
        let tree = self.borrow();
        for child_node in tree.children.iter()
        {
            result.append(&mut child_node.flatten_branches());
        }
        if !self.is_leaf(){
            result.push(tree.value.clone());
        }
        result
    }
}

impl<T> Deref for Node<T>
{
    type Target = RefCell<Tree<T>>;

    fn deref(&self) -> &Self::Target {
       self.0.deref()
    }
}

impl<T> Clone for Node<T>
{
    fn clone(&self) -> Self {
        Node(Rc::clone(&self.0))
    }
}

#[derive(Debug, Clone)]
pub struct Tree<T>
{
    pub value : T,
    pub children : Vec<Node<T>>
}

impl<T> Tree<T>
{
    pub fn new(val : T) -> Tree<T>
    {
        Tree
        {
            value : val,
            children : vec![]
        }
    }


}


pub fn find_node<T>(t : Node<T>, val : &T) -> Option<Node<T>>
where T : Eq
{
    let tb = t.borrow();
    if tb.value == *val {
        return Some(Node::clone(&t));
    }
    
    for b in tb.children.iter()
    {
        if let Some(r) = find_node(Node::clone(&b),val) {
            return Some(Node::clone(&r));
        }
    }

    None
}