use crate::ten_q::MAX_LEVELS_UP;
use core::borrow::{Borrow, BorrowMut};
use html5ever::rcdom::{Handle, Node, NodeData, RcDom};
use html5ever::tendril::{SliceExt, StrTendril, TendrilSink};
use html5ever::{LocalName, QualName};
use markup5ever::Attribute;
use std::cell::RefCell;
use std::fs::File;
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
    let parent = target.parent.take();
    target.parent.set(parent.clone());
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

pub fn get_parents_and_indexes(handle: &Handle) -> Vec<(Rc<Node>, i32)> {
    let immediate_parent_and_index =
        get_parent_and_index(handle).expect("Should have an immediate parent!");
    // Seed the vector with the immediate parent to make the loop logic below work well.
    let mut parents_and_indexes: Vec<(Rc<Node>, i32)> = vec![immediate_parent_and_index];

    // get parents several levels up:
    for i in 1..=MAX_LEVELS_UP {
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

pub fn add_attribute(handle: &Handle, new_attr: Attribute, strip_attr: Option<&'static str>) {
    match handle.data {
        NodeData::Element { ref attrs, .. } => {
            if let Some(n) = strip_attr {
                attrs.borrow_mut().retain(|attr| &attr.name.local != n);
            }
            attrs.borrow_mut().push(new_attr);
        }
        _ => panic!("Node should be an element!"),
    }
}

pub fn create_x_birb_attr(name: &'static str) -> Attribute {
    Attribute {
        name: QualName::new(
            None,
            ns!(),
            LocalName::from("x-birb-income-statement-table"),
        ),
        value: "".to_tendril(),
    }
}

pub fn write_to_file(file_path: &String, data: Vec<u8>) -> std::io::Result<()> {
    let absolute_path = get_abs_path(file_path);
    let mut pos = 0;
    let mut buffer = File::create(absolute_path).expect("Couldn't make file");

    while pos < data.len() {
        let bytes_written = buffer.write(&data[pos..])?;
        pos += bytes_written;
    }
    Ok(())
}
