use std::cell::RefCell;
use std::rc::Rc;

use crate::node::Node;

#[derive(Debug)]
pub struct Tree {
    root: Option<Node>,

}

impl Tree {
    pub fn new() -> Tree {
        Tree {
            root: None,

        }
    }


    pub fn add_value(&mut self, val: f32) {
        let n: Node = Node::new(val);
        if self.root.is_none() {
            self.root = Some(n);
        } else {
//            let kk = self.root.as_mut();
//            let hh: &mut Node = kk.unwrap();
//            hh.add_node(n);
            self.root.as_mut().unwrap().add_node(n);
        }
    }
}