extern crate filing_parser;
#[macro_use]
extern crate lazy_static;

use filing_parser::helpers::{bfs_find_node, tendril_to_string, write_to_file};
use filing_parser::test_files::get_files;
use regex::{Captures, Regex, RegexBuilder};
use std::fs::remove_dir;
use std::rc::Rc;
use xml5ever::driver::parse_document;
use xml5ever::rcdom::{Handle, Node, NodeData, RcDom};
use xml5ever::serialize::serialize;
use xml5ever::tendril::TendrilSink;

lazy_static! {
    static ref ESCAPE_GRAPHIC_PATTERN: &'static str = r"
       (
            (?:<TYPE>GRAPHIC)
            .*?
        )
        (?:<TEXT>)
        (.*?)
        (?:</TEXT>)
    ";
    pub static ref ESCAPE_GRAPHIC_REGEX: Regex = RegexBuilder::new(&ESCAPE_GRAPHIC_PATTERN)
        .case_insensitive(true)
        .multi_line(true)
        .ignore_whitespace(true)
        .dot_matches_new_line(true)
        .build()
        .expect("Couldn't build graph contents regex!");
    static ref UNESCAPE_GRAPHIC_PATTERN: &'static str = r"
       (.*)
    ";
    pub static ref UNESCAPE_GRAPHIC_REGEX: Regex = RegexBuilder::new(&UNESCAPE_GRAPHIC_PATTERN)
        .case_insensitive(true)
        .multi_line(true)
        .ignore_whitespace(true)
        .dot_matches_new_line(true)
        .build()
        .expect("Couldn't build graph contents regex!");
}

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
        //        include_str!("../../examples/10-Q/input/0001144204-16-084770.txt"),
    ];
    for file_contents in test_files {
        let file_contents = escape_graphic_node_contents(file_contents);

        let dom: RcDom = parse_document(RcDom::default(), Default::default()).one(&*file_contents);
        let document: &Rc<Node> = &dom.document;
        assert_eq!(
            1,
            document.children.borrow().len(),
            "There should only be one main node!"
        );
        let main_node = &document.children.borrow()[0];
        assert_main_node_is_SEC_DOCUMENT(main_node);
        let parsed_documents = split_document_into_documents(main_node);
        let unescaped_parsed_documents = unescape_parsed_documents(parsed_documents);

        write_parsed_docs_to_example_folder(unescaped_parsed_documents);
    }
}

fn unescape_parsed_documents(parsed_docs: Vec<ParsedDocument>) -> Vec<ParsedDocument> {
    let mut escaped_parsed_docs = vec![];
    for mut doc in parsed_docs {
        doc.text = String::from(unescape_graphic_node_contents(&doc.text));
        escaped_parsed_docs.push(doc);
    }
    escaped_parsed_docs
}

fn split_document_into_documents(main_node: &Handle) -> Vec<ParsedDocument> {
    main_node
        .children
        .borrow()
        .iter()
        .filter_map(|node| parse_doc(node))
        .collect::<Vec<ParsedDocument>>()
}

fn escape_graphic_node_contents(contents: &str) -> String {
    String::from(
        ESCAPE_GRAPHIC_REGEX.replace_all(contents, |caps: &Captures| {
            format!(
                r#"{}<TEXT>{}</TEXT>"#,
                &caps[1],
                &caps[2]
                    .replace("&", "&amp;")
                    .replace("<", "&lt;")
                    .replace("\"", "&quot;")
                    .replace("'", "&apos;")
                    .replace(">", "&gt;")
            )
        }),
    )
}

fn unescape_graphic_node_contents(contents: &str) -> String {
    String::from(
        UNESCAPE_GRAPHIC_REGEX.replace_all(contents, |caps: &Captures| {
            format!(
                "{}",
                &caps[0]
                    .replace("&amp;", "&")
                    .replace("&lt;", "<")
                    .replace("&quot;", "\"")
                    .replace("&apos;", "'")
                    .replace("&gt;", ">")
            )
        }),
    )
}

fn write_parsed_docs_to_example_folder(parsed_documents: Vec<ParsedDocument>) {
    for doc in parsed_documents {
        write_to_file(
            &String::from(format!(
                "/Users/murtyjones/Desktop/example-parsed/{}",
                doc.filename,
            )),
            "",
            doc.text.as_bytes().to_owned(),
        )
        .expect("Couldn't write to file!");
    }
}

fn assert_main_node_is_SEC_DOCUMENT(handle: &Handle) {
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

            let type_node =
                bfs_find_node(Rc::clone(handle), |node: Handle| find_element(node, "TYPE"))
                    .expect("No TYPE node found!");

            let sequence_node = bfs_find_node(Rc::clone(handle), |node: Handle| {
                find_element(node, "SEQUENCE")
            })
            .expect("No SEQUENCE node found!");

            let filename_node = bfs_find_node(Rc::clone(handle), |node: Handle| {
                find_element(node, "FILENAME")
            })
            .expect("No FILENAME node found!");

            // There does not have to be a description node
            let description_node = bfs_find_node(Rc::clone(handle), |node: Handle| {
                find_element(node, "DESCRIPTION")
            });

            let text_node =
                bfs_find_node(Rc::clone(handle), |node: Handle| find_element(node, "TEXT"))
                    .expect("No TEXT node found!");

            let type_contents = get_node_contents_as_str(&type_node);
            let description_contents = match description_node {
                Some(d) => Some(get_node_contents_as_str(&d)),
                None => None,
            };
            let sequence_contents = get_node_contents_as_int(&sequence_node);
            let filename_contents = get_node_contents_as_str(&filename_node);
            let mut text_contents = get_text_node_children(&text_node);

            Some(ParsedDocument {
                type_: type_contents,
                sequence: sequence_contents,
                filename: filename_contents,
                description: description_contents,
                text: text_contents,
            })
        }
        _ => None,
    }
}

#[derive(Debug)]
pub struct ParsedDocument {
    /// The type of document (e.g. 10-Q). "type" is a Rust reserved keyword. Hence the underscore
    pub type_: String,
    /// The sequence of the document for display purposes. 1 is the most important
    pub sequence: i32,
    /// The filename of the document (e.g. d490575d10q.htm)
    pub filename: String,
    /// The SEC's description of the document (e.g. FORM 10-Q)
    pub description: Option<String>,
    /// The actual document contents
    pub text: String,
}

fn find_element(node: Handle, target_name: &'static str) -> Option<Handle> {
    if let NodeData::Element { ref name, .. } = node.data {
        if &name.local == target_name {
            return Some(Rc::clone(&node));
        }
    }
    None
}

fn get_node_contents_as_str(node: &Rc<Node>) -> String {
    if let NodeData::Element { ref name, .. } = node.data {
        assert!(
            0 < node.children.borrow().len(),
            "Node should have children!"
        );
        if let NodeData::Text { ref contents } = &node.children.borrow()[0].data {
            return String::from(tendril_to_string(contents).trim());
        }
        panic!("First child is not a text node!")
    }
    panic!("Wrong node type!")
}

fn get_node_contents_as_int(node: &Rc<Node>) -> i32 {
    if let NodeData::Element { ref name, .. } = node.data {
        assert!(
            0 < node.children.borrow().len(),
            "Node should have children!"
        );
        if let NodeData::Text { ref contents } = &node.children.borrow()[0].data {
            let as_str = tendril_to_string(contents);
            let as_int: i32 = as_str.trim().parse().unwrap();
            return as_int;
        }
        panic!("First child is not a text node!")
    }
    panic!("Wrong node type!")
}

fn serialize_node(node: &Rc<Node>) -> String {
    let mut bytes = vec![];
    xml5ever::serialize::serialize(
        &mut bytes,
        node,
        xml5ever::serialize::SerializeOpts::default(),
    )
    .expect("Couldn't write to file.");
    return String::from_utf8(bytes).unwrap().trim().to_string();
}

fn get_text_node_children(node: &Rc<Node>) -> String {
    if let xml5ever::rcdom::NodeData::Element { ref name, .. } = node.data {
        assert_eq!(
            "TEXT", &name.local,
            "No text found! Expected node was not <TEXT>"
        );
        assert!(
            0 < node.children.borrow().len(),
            "Text node had no children!"
        );
        return serialize_node(&node);
    }

    panic!("No text found! Expected node was not an Element.");
}

mod test {
    use super::*;

    #[test]
    fn test_escape_graphic_node_contents() {
        let contents = r#"
        some
        other
        unimportant
        stuff
<TYPE>GRAPHIC
<SEQUENCE>6
<FILENAME>image1.gif
<DESCRIPTION>MATERIAL WEAKNESS REMEDIATION GRAPHIC
<TEXT><</TEXT>
    some
        nonsense
            after the fact
        "#;

        let expected_contents = r#"
        some
        other
        unimportant
        stuff
<TYPE>GRAPHIC
<SEQUENCE>6
<FILENAME>image1.gif
<DESCRIPTION>MATERIAL WEAKNESS REMEDIATION GRAPHIC
<TEXT>&lt;</TEXT>
    some
        nonsense
            after the fact
        "#;
        let r = escape_graphic_node_contents(contents);
        assert_eq!(expected_contents, r);
    }

    #[test]
    fn test_unescape_graphic_node_contents() {
        let contents = r#"
        some
        other
        unimportant
        stuff
<TYPE>GRAPHIC
<SEQUENCE>6
<FILENAME>image1.gif
<DESCRIPTION>MATERIAL WEAKNESS REMEDIATION GRAPHIC
<TEXT>&lt;</TEXT>
    some
        nonsense
            after the fact
        "#;
        let expected_contents = r#"
        some
        other
        unimportant
        stuff
<TYPE>GRAPHIC
<SEQUENCE>6
<FILENAME>image1.gif
<DESCRIPTION>MATERIAL WEAKNESS REMEDIATION GRAPHIC
<TEXT><</TEXT>
    some
        nonsense
            after the fact
        "#;
        let r = unescape_graphic_node_contents(contents);
        assert_eq!(expected_contents, r);
    }
}
