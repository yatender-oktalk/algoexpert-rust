use std::cmp::Ordering;

use crate::tree::node::Node;

#[allow(dead_code)]
pub struct Tree<T: Ord> {
    root: Option<Box<Node<T>>>,
}

pub trait TreeFunctions<T> {
    fn add(&mut self, val: T) -> &mut Self;
}

#[allow(dead_code)]
impl<T: Ord> Tree<T> {
    pub fn new() -> Self {
        Self {root: None}
    }

    pub fn with_root(value: T) -> Self {
        Self {
            root: Node::new(value)
        }
    }
}

impl<T: Ord> Default for Tree<T> {
    fn default() -> Self {
        Tree::new()
    }
}

impl <T: Ord> TreeFunctions<T> for Tree<T> {
    fn add(&mut self, val: T) -> &mut Self {
        match &mut self.root {
            None => {
                self.root = Node::new(val);
            }
            Some(root) => {
                let mut current = root;
                loop {
                    match val.cmp(&current.data) {
                        Ordering::Equal => {
                            break;
                        }
                        Ordering::Less => {
                            match current.left {
                                None => {
                                    current.left = Node::new(val);
                                    break;
                                }
                                Some(ref mut next) => {
                                    current = next;
                                }
                            }
                        }
                        Ordering::Greater => {
                            match current.right {
                                None => {
                                    current.right = Node::new(val);
                                    break;
                                }    
                                Some(ref mut next) => {
                                    current = next
                                }
                            }
                        }
                    }
                }
            }
        }
        self
    }
}