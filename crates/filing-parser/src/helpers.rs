use core::borrow::Borrow;
use html5ever::rcdom::{Handle, Node, NodeData};
use html5ever::tendril::{SliceExt, StrTendril};
use html5ever::{LocalName, QualName};
use markup5ever::Attribute;
use regex::Regex;
use std::cell::RefCell;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::rc::Rc;

pub fn get_abs_path(rel_path: &String) -> PathBuf {
    let relative_path = std::path::PathBuf::from(rel_path);
    let mut absolute_path = std::env::current_dir().expect("Need current dir!");
    absolute_path.push(relative_path);
    absolute_path
}

pub fn path_exists(file_path: &String) -> bool {
    Path::new(file_path).exists()
}

pub fn same_node(x: &Handle, y: &Handle) -> bool {
    // FIXME: This shouldn't really need to touch the borrow flags, right?
    (&*x.borrow() as *const Node) == (&*y.borrow() as *const Node)
}

pub fn get_parent_and_index(target: &Handle) -> Option<(Handle, i32)> {
    // TODO this will hopefully be fixed soon and .get() can be used instead:
    let parent = target
        .parent
        .take()
        .expect("No parent!")
        .upgrade()
        .expect("dangling weak pointer!");
    let num2 = Rc::clone(&parent);
    let weaknum2 = std::rc::Rc::downgrade(&num2);
    target.parent.set(Some(weaknum2));
    let children = parent.children.borrow();

    match children
        .iter()
        .enumerate()
        .find(|&(_, n)| same_node(n, target))
    {
        Some((i, _)) => Some((Rc::clone(&parent), i as i32)),
        None => panic!("Have parent but couldn't find in parent's children!"),
    }
}

pub fn get_parents_and_indexes(handle: &Handle) -> Vec<(Rc<Node>, i32)> {
    let immediate_parent_and_index =
        get_parent_and_index(handle).expect("Should have an immediate parent!");
    // Seed the vector with the immediate parent to make the loop logic below work well.
    let mut parents_and_indexes: Vec<(Rc<Node>, i32)> = vec![immediate_parent_and_index];
    // get parents several levels up:
    for i in 1..=5 {
        let prev_node_index = (i as usize) - 1;
        let prev_node = &parents_and_indexes[prev_node_index].0;
        parents_and_indexes
            .push(get_parent_and_index(prev_node).expect("Couldn't get parent node and index."));
    }

    parents_and_indexes
}

pub fn tendril_to_string(text: &RefCell<StrTendril>) -> String {
    let mut converted = String::new();
    converted.push_str(&text.borrow());
    converted
}

pub fn get_children(handle: &Handle) -> Vec<Handle> {
    handle
        .children
        .borrow()
        .iter()
        .filter(|child| match child.data {
            NodeData::Element { .. } | NodeData::Text { .. } => true,
            _ => false,
        })
        .map(|child| Rc::clone(child))
        .collect::<Vec<Rc<Node>>>()
}

pub fn bfs<CB>(handle: Handle, mut cb: CB) -> bool
where
    CB: (FnMut(Handle) -> bool),
{
    let mut q = vec![handle];
    while q.len() > 0 {
        let node = q.remove(0);
        if cb(Rc::clone(&node)) {
            return true;
        }
        q.append(&mut get_children(&node));
    }
    false
}
pub fn bfs_find_node<CB>(handle: Handle, mut cb: CB) -> Option<Handle>
where
    CB: (Fn(Handle) -> Option<Handle>),
{
    let mut q = vec![handle];
    while q.len() > 0 {
        let node = q.remove(0);
        if let Some(h) = cb(Rc::clone(&node)) {
            return Some(h);
        }
        q.append(&mut get_children(&node));
    }
    None
}

fn prepend<T>(v: Vec<T>, s: &[T]) -> Vec<T>
where
    T: Clone,
{
    let mut tmp: Vec<_> = s.to_owned();
    tmp.extend(v);
    tmp
}
