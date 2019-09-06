extern crate filing_parser;
#[macro_use]
extern crate lazy_static;
extern crate models;
extern crate uuencode;

use filing_parser::helpers::{bfs_find_node, tendril_to_string, write_to_file};
use filing_parser::test_files::get_files;
use models::SplitDocument;
use regex::{Captures, Regex, RegexBuilder};
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

fn full_submission_splitter(file_contents: &str, filing_id: i32) -> Vec<SplitDocument> {
    let file_contents = escape_graphic_node_contents(file_contents);

    let dom: RcDom = parse_document(RcDom::default(), Default::default()).one(&*file_contents);
    let document: &Rc<Node> = &dom.document;
    assert_eq!(
        1,
        document.children.borrow().len(),
        "There should only be one main node!"
    );
    let sec_document_node = &document.children.borrow()[0];
    assert_sec_document_node_is_sec_document(sec_document_node);
    let parsed_documents = split_document_into_documents(sec_document_node, filing_id);
    unescape_parsed_documents(parsed_documents)
}

/// Applies the unescape logic to all the GRAPHIC documents, then returns all parsed documents.
fn unescape_parsed_documents(parsed_docs: Vec<SplitDocument>) -> Vec<SplitDocument> {
    let mut escaped_parsed_docs = vec![];
    for mut doc in parsed_docs {
        if doc.doc_type == "GRAPHIC" {
            doc.text = String::from(unescape_node_contents(&doc.text));
        }
        escaped_parsed_docs.push(doc);
    }
    escaped_parsed_docs
}

/// Takes the <SEC-DOCUMENT> node of the document and tries to parse its children
/// into seperate documents. Most of its children are <DOCUMENT> nodes, which
/// should be parsable. Some however are things like <SEC-HEADER>, which is not
/// a document. Hence the filter_map.
fn split_document_into_documents(sec_document_node: &Handle, filing_id: i32) -> Vec<SplitDocument> {
    sec_document_node
        .children
        .borrow()
        .iter()
        // Filter to only the parsed documents that succeeded:
        .filter_map(|node| parse_doc(node, filing_id))
        .collect::<Vec<SplitDocument>>()
}

/// Escape's a node's contents so that it can be correctly parsed as XML.
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

/// Unescape's the node's contents that were previously escaped.
fn unescape_node_contents(contents: &str) -> String {
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

/// Ensures that we grabbed the <SEC-DOCUMENT> node, which we need
/// to use as a base to parse any document.
fn assert_sec_document_node_is_sec_document(handle: &Handle) {
    match handle.data {
        NodeData::Element { ref name, .. } => {
            assert!("SEC-DOCUMENT" == &name.local, "First node in the document is not named '<SEC-DOCUMENT>'!");
        }
        _ => panic!("First node in the document is not an element. It should be an element with the name '<SEC-DOCUMENT>'")
    }
}

/// If a node is a <DOCUMENT>, parses its components into a SplitDocument.
/// If it's an <SEC-HEADER>, returns None
fn parse_doc(handle: &Rc<Node>, filing_id: i32) -> Option<SplitDocument> {
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
            let text_contents = get_text_node_children(&text_node);

            Some(SplitDocument {
                filing_id,
                doc_type: type_contents,
                sequence: sequence_contents,
                filename: filename_contents,
                description: description_contents,
                text: text_contents,
                created_at: None,
                updated_at: None,
            })
        }
        _ => None,
    }
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
    if let NodeData::Element { .. } = node.data {
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
    if let NodeData::Element { .. } = node.data {
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
    serialize(
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

    /// Converts docs' contents to bytes (which may include uudecoding)
    /// and writes them to an example folder locally.
    fn write_parsed_docs_to_example_folder(parsed_documents: Vec<SplitDocument>) {
        for doc in parsed_documents {
            let mut contents_for_file = doc.text.as_bytes().to_owned();
            if doc.doc_type == "GRAPHIC" {
                //            panic!("{:?}", doc.text);
                contents_for_file = uuencode::uudecode(&*doc.text)
                    .expect("Couldn't uudecode document contents!")
                    .0;
            }
            write_to_file(
                &String::from(format!(
                    "/Users/murtyjones/Desktop/example-parsed/{}",
                    doc.filename,
                )),
                "",
                contents_for_file,
            )
            .expect("Couldn't write to file!");
        }
    }

    #[test]
    fn test_local_file_1() {
        // Chosen at random
        let file_contents = include_str!("../../examples/10-Q/input/0001004434-17-000011.txt");
        let stub_id = 1;
        let r = full_submission_splitter(file_contents, stub_id);
        assert_eq!(83, r.len());
        //        write_parsed_docs_to_example_folder(r);
    }

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
    fn test_unescape_node_contents() {
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
        let r = unescape_node_contents(contents);
        assert_eq!(expected_contents, r);
    }
}
