use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    val: f32,

    pub left: Option<Rc<Node>>,
    pub right: Option<Rc<Node>>,
}

impl Node {
    pub fn new(val: f32) -> Node {
        Node {
            val,
            left: None,
            right: None,
        }
    }

    pub fn add_node(&mut self, n: Node) {
        if n.val < self.val {
            if self.left.is_none() {
                self.left = Some(Rc::new(n));
                println!("En 1");
            } else {
                let kk = self.left.as_ref();
                let hh = kk.unwrap();
                let bb = &mut *hh.clone();
                //let ww: f32 = *bb;
                bb.add_node(n);
                //let kk = self.left.as_ref();
                //let kk = self.left.as_ref();
                //let mut hh = kk.unwrap();
                //let hh = kk.unwrap();
                //let ss = &mut **hh;
                //let bb = *hh;
                //let ww: f32 = *bb;
                //bb.add_node(n);
                //let yy: f32 = bb;
                //dbg!(hh);
                //ss.add_node(n);
            }
            //self.left = Some(Rc::new(n));
            //let rc = self.left.as_mut();
            //rc.into_inner().add_node(n);
            //self.left.as_mut().unwrap().add_node(n);
        } else {
            if self.left.is_none() {
                self.right = Some(Rc::new(n));
                println!("En 2");
            } else {
                let kk = self.right.as_mut();
                let mut hh = kk.unwrap();
                dbg!(hh);
                //hh.add_node(n);
            }
        }
    }
}
