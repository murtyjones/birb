use core::borrow::Borrow;
use html5ever::rcdom::{Handle, Node, NodeData};
use html5ever::tendril::{SliceExt, StrTendril};
use html5ever::{LocalName, QualName};
use markup5ever::Attribute;
use regex::Regex;
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

pub fn bfs_with_matches<CB>(handle: Handle, mut cb: CB) -> i32
where
    CB: (FnMut(Handle) -> Option<&'static Regex>),
{
    let mut matches: Vec<&'static Regex> = vec![];
    let mut q = vec![handle];
    while q.len() > 0 {
        let node = q.remove(0);
        if let Some(r) = cb(Rc::clone(&node)) {
            let is_already_found = matches.iter().fold(false, |acc, each| {
                if each.as_str() == r.as_str() {
                    return true;
                }
                return acc;
            });
            if !is_already_found {
                matches.push(r);
            }
        }
        // Prepend the child's elements to the queue. This is less
        // ideal than appending them because it requires more memory,
        // but we need to parse the document in order (IE from top to
        // bottom), so this approach is needed for now.
        q = prepend(q, &mut get_children(&node));
    }
    matches.len() as i32
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
        // Prepend the child's elements to the queue. This is less
        // ideal than appending them because it requires more memory,
        // but we need to parse the document in order (IE from top to
        // bottom), so this approach is needed for now.
        q = prepend(q, &mut get_children(&node));
    }
    false
}

pub fn bfs_no_base_case<CB>(handle: Handle, mut cb: CB) -> ()
where
    CB: (FnMut(Handle) -> bool),
{
    let mut q = vec![handle];
    while q.len() > 0 {
        let node = q.remove(0);
        let _found = cb(Rc::clone(&node));
        // Prepend the child's elements to the queue. This is less
        // ideal than appending them because it requires more memory,
        // but we need to parse the document in order (IE from top to
        // bottom), so this approach is needed for now.
        q = prepend(q, &mut get_children(&node));
    }
}

pub fn bfs_skip_chillins<CB>(handle: Handle, mut cb: CB) -> ()
where
    CB: (FnMut(Handle) -> bool),
{
    let mut q = vec![handle];
    while q.len() > 0 {
        let node = q.remove(0);
        let found = cb(Rc::clone(&node));
        if !found {
            // Prepend the child's elements to the queue. This is less
            // ideal than appending them because it requires more memory,
            // but we need to parse the document in order (IE from top to
            // bottom), so this approach is needed for now.
            q = prepend(q, &mut get_children(&node));
        }
    }
}

fn prepend<T>(v: Vec<T>, s: &[T]) -> Vec<T>
where
    T: Clone,
{
    let mut tmp: Vec<_> = s.to_owned();
    tmp.extend(v);
    tmp
}
