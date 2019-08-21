extern crate filing_parser;

use filing_parser::helpers::{bfs_find_node, tendril_to_string, write_to_file};
use filing_parser::test_files::get_files;
use std::rc::Rc;
use xml5ever::driver::parse_document;
use xml5ever::rcdom::{Handle, Node, NodeData, RcDom};
use xml5ever::serialize::serialize;
use xml5ever::tendril::TendrilSink;

/// Splits a full submission text file into its parts
fn main() {
    let test_files: Vec<&'static str> = vec![
        //        include_str!("../../examples/10-Q/input/0001000623-17-000125.txt"),
        //        include_str!("../../examples/10-Q/input/0001001288-16-000069.txt"),
        include_str!("../../examples/10-Q/input/0001004434-17-000011.txt"),
        //        include_str!("../../examples/10-Q/input/0001004980-16-000073.txt"),
        //        include_str!("../../examples/10-Q/input/0001015780-17-000075.txt"),
        //        include_str!("../../examples/10-Q/input/0001079973-17-000690.txt"),
        //        include_str!("../../examples/10-Q/input/0001185185-16-005721.txt"),
        //        include_str!("../../examples/10-Q/input/0001185185-16-005747.txt"),
        //        include_str!("../../examples/10-Q/input/0001193125-16-454777.txt"),
        //        include_str!("../../examples/10-Q/input/0001193125-17-160261.txt"),
        //        include_str!("../../examples/10-Q/input/0001193125-18-037381.txt"),
        //        include_str!("../../examples/10-Q/input/0001213900-16-018375.txt"),
        //        include_str!("../../examples/10-Q/input/0001437749-16-025027.txt"),
        //        include_str!("../../examples/10-Q/input/0001437749-16-036870.txt"),
        //        include_str!("../../examples/10-Q/input/0001493152-17-009297.txt"),
        //        include_str!("../../examples/10-Q/input/0001564590-17-009385.txt"),
    ];
    for file_contents in test_files {
        let dom: RcDom = parse_document(RcDom::default(), Default::default()).one(&*file_contents);
        let document: &Rc<Node> = &dom.document;
        assert_eq!(
            1,
            document.children.borrow().len(),
            "There should only be one main node!"
        );
        let main_node = &document.children.borrow()[0];
        assert_main_node_is_sec_document(main_node);
        let parsed_documents = main_node
            .children
            .borrow()
            .iter()
            .map(|node| parse_doc(node))
            .collect::<Vec<Option<ParsedDocument>>>();

        for doc in &parsed_documents {
            if let Some(d) = doc {
                write_to_file(
                    &String::from(format!(
                        "/Users/murtyjones/Desktop/example-parsed/{}",
                        d.filename
                    )),
                    d.text.as_bytes().to_owned(),
                )
                .expect("Couldn't write to file!");
            }
        }
    }
}

fn assert_main_node_is_sec_document(handle: &Handle) {
    match handle.data {
        NodeData::Element { ref name, .. } => {
            assert!("SEC-DOCUMENT" == &name.local, "First node in the document is not named '<SEC-DOCUMENT>'!");
        }
        _ => panic!("First node in the document is not an element. It should be an element with the name '<SEC-DOCUMENT>'")
    }
}

fn parse_doc(handle: &Rc<Node>) -> Option<ParsedDocument> {
    match handle.data {
        NodeData::Element { ref name, .. } => {
            if "SEC-HEADER" == &name.local {
                return None;
            }
            assert_eq!(
                "DOCUMENT", &name.local,
                "Node should be a document element!"
            );

            let type_node = bfs_find_node(Rc::clone(handle), find_type_node);

            let type_node = &Rc::clone(&handle.children.borrow()[1]);
            let sequence_node = &type_node.children.borrow()[1];
            let filename_node = &sequence_node.children.borrow()[1];
            let description_node = &filename_node.children.borrow()[1];
            let text_node = &description_node.children.borrow()[1];
            Some(ParsedDocument {
                type_: get_doc_type(type_node),
                sequence: get_doc_sequence(sequence_node),
                filename: get_doc_filename(filename_node),
                description: get_doc_description(description_node),
                text: get_doc_text(text_node),
            })
        }
        _ => None,
    }
}

pub struct ParsedDocument {
    /// The type of document (e.g. 10-Q). "type" is a Rust reserved keyword. Hence the underscore
    pub type_: String,
    /// The sequence of the document for display purposes. 1 is the most important
    pub sequence: i32,
    /// The filename of the document (e.g. d490575d10q.htm)
    pub filename: String,
    /// The SEC's description of the document (e.g. FORM 10-Q)
    pub description: String,
    /// The actual document contents
    pub text: String,
}

fn find_type_node(node: Handle) -> Option<Handle> {
    find_element(node, "TYPE")
}

fn find_element(node: Handle, target_name: &'static str) -> Option<Handle> {
    if let NodeData::Element { ref name, .. } = node.data {
        if &name.local == target_name {
            return Some(Rc::clone(&node));
        }
    }
    None
}

fn get_doc_type(node: &Rc<Node>) -> String {
    if let NodeData::Element { ref name, .. } = node.data {
        assert_eq!("TYPE", &name.local);
        assert!(0 < node.children.borrow().len());
        if let NodeData::Text { ref contents } = &node.children.borrow()[0].data {
            return String::from(tendril_to_string(contents).trim());
        }
        panic!("Doc type not found! No text node.");
    }
    panic!("Doc type not found!");
}

fn get_doc_sequence(node: &Rc<Node>) -> i32 {
    if let NodeData::Element { ref name, .. } = node.data {
        assert_eq!("SEQUENCE", &name.local);
        assert!(0 < node.children.borrow().len());
        if let NodeData::Text { ref contents } = &node.children.borrow()[0].data {
            let as_str = tendril_to_string(contents);
            let as_int: i32 = as_str.trim().parse().unwrap();
            return as_int;
        }
        panic!("No sequence found!");
    }
    panic!("No sequence found!");
}

fn get_doc_filename(node: &Rc<Node>) -> String {
    if let NodeData::Element { ref name, .. } = node.data {
        assert_eq!("FILENAME", &name.local);
        assert!(0 < node.children.borrow().len());
        if let NodeData::Text { ref contents } = &node.children.borrow()[0].data {
            let as_str = String::from(tendril_to_string(contents).trim());
            return as_str;
        }
        panic!("No filename found!");
    }
    panic!("No filename found!");
}

fn get_doc_description(node: &Rc<Node>) -> String {
    if let NodeData::Element { ref name, .. } = node.data {
        assert_eq!("DESCRIPTION", &name.local);
        assert!(0 < node.children.borrow().len());
        if let NodeData::Text { ref contents } = &node.children.borrow()[0].data {
            let as_str = String::from(tendril_to_string(contents).trim());
            return as_str;
        }
        panic!("No description found!");
    }
    panic!("No description found!");
}

fn ser(node: &Rc<Node>) -> String {
    let mut bytes = vec![];
    xml5ever::serialize::serialize(
        &mut bytes,
        node,
        xml5ever::serialize::SerializeOpts::default(),
    )
    .expect("Couldn't write to file.");
    return String::from_utf8(bytes).unwrap();
}

fn get_doc_text(node: &Rc<Node>) -> String {
    if let xml5ever::rcdom::NodeData::Element { ref name, .. } = node.data {
        assert_eq!(
            "TEXT", &name.local,
            "No text found! Expected node was not <TEXT>"
        );
        assert!(
            0 < node.children.borrow().len(),
            "Text node had no children!"
        );
        return ser(&node);
    }

    panic!("No text found! Expected node was not an Element.");
}
