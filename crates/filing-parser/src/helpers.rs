use core::borrow::{Borrow, BorrowMut};
use html5ever::rcdom::{Handle, Node, RcDom};
use std::rc::Rc;

//macro_rules! unwrap_or_return {
//    ( $e:expr ) => {
//        match $e {
//            Ok(x) => x,
//            Err(_) => return,
//        }
//    };
//}

pub fn same_node(x: &Handle, y: &Handle) -> bool {
    // FIXME: This shouldn't really need to touch the borrow flags, right?
    (&*x.borrow() as *const Node) == (&*y.borrow() as *const Node)
}

pub fn get_parent_and_index(target: &Handle) -> Option<(Handle, i32)> {
    // TODO this will hopefully be fixed soon:
    let parent = target.parent.take();
    match parent {
        Some(n) => {
            let parent = n.upgrade().expect("dangling weak pointer");
            let children = &parent.children.borrow();
            match children
                .iter()
                .enumerate()
                .find(|&(_, n)| same_node(n, target))
            {
                Some((i, _)) => Some((Rc::clone(&parent), i as i32)),
                None => panic!("Have parent but couldn't find in parent's children!"),
            }
        }
        None => panic!("No parent!"),
    }
}
