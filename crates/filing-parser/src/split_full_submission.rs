use crate::helpers::{bfs_find_node, tendril_to_string};
use crate::test_files::get_files;
use models::SplitDocumentBeforeUpload;
use regex::{Captures, Regex, RegexBuilder};
use std::rc::Rc;
use utils::write_to_file;
use xml5ever::driver::parse_document;
use xml5ever::rcdom::{Handle, Node, NodeData, RcDom};
use xml5ever::serialize::serialize;
use xml5ever::tendril::TendrilSink;

/// Document types that need to be unescaped and decoded
const UUENCODED_DOCUMENTS: [&str; 3] = ["GRAPHIC", "EXCEL", "ZIP"];

lazy_static! {
    static ref ESCAPE_ENCODED_PATTERN: &'static str = r"
       (
            (?:<TYPE>(?:GRAPHIC|EXCEL|ZIP))
            .*?
        )
        (?:<TEXT>)
        (.*?)
        (?:</TEXT>)
    ";
    pub static ref ESCAPE_ENCODED_REGEX: Regex = RegexBuilder::new(&ESCAPE_ENCODED_PATTERN)
        .case_insensitive(true)
        .multi_line(true)
        .ignore_whitespace(true)
        .dot_matches_new_line(true)
        .build()
        .expect("Couldn't build graph contents regex!");
    static ref UNESCAPE_ENCODED_PATTERN: &'static str = r"
       (.*)
    ";
    pub static ref UNESCAPE_ENCODED_REGEX: Regex = RegexBuilder::new(&UNESCAPE_ENCODED_PATTERN)
        .case_insensitive(true)
        .multi_line(true)
        .ignore_whitespace(true)
        .dot_matches_new_line(true)
        .build()
        .expect("Couldn't build graph contents regex!");
}

pub fn split_full_submission(file_contents: &str) -> Vec<SplitDocumentBeforeUpload> {
    let file_contents = escape_encoded_node_contents(file_contents);

    let dom: RcDom = parse_document(RcDom::default(), Default::default()).one(&*file_contents);
    let document: &Rc<Node> = &dom.document;
    assert_eq!(
        1,
        document.children.borrow().len(),
        "There should only be one main node!"
    );
    let sec_document_node = &document.children.borrow()[0];
    assert_sec_document_node_is_sec_document(sec_document_node);
    let parsed_documents = split_main_node_into_documents(sec_document_node);
    unescape_parsed_documents(parsed_documents)
}

/// Applies the unescape logic to all the uuencoded documents, then returns all parsed documents.
fn unescape_parsed_documents(
    parsed_docs: Vec<SplitDocumentBeforeUpload>,
) -> Vec<SplitDocumentBeforeUpload> {
    let mut escaped_parsed_docs = vec![];
    for mut doc in parsed_docs {
        if UUENCODED_DOCUMENTS.contains(&&*doc.doc_type) {
            doc.text = String::from(unescape_node_contents(&doc.text));
            doc.decoded_text = Some(uuencode::uudecode(&*doc.text).expect("couldn't decode!").0);
        }
        escaped_parsed_docs.push(doc);
    }
    escaped_parsed_docs
}

/// Takes the <SEC-DOCUMENT> node of the document and tries to parse its children
/// into seperate documents. Most of its children are <DOCUMENT> nodes, which
/// should be parsable. Some however are things like <SEC-HEADER>, which is not
/// a document. Hence the filter_map.
fn split_main_node_into_documents(sec_document_node: &Handle) -> Vec<SplitDocumentBeforeUpload> {
    sec_document_node
        .children
        .borrow()
        .iter()
        // Filter to only the parsed documents that succeeded:
        .filter_map(|node| parse_doc(node))
        .collect::<Vec<SplitDocumentBeforeUpload>>()
}

/// Escape's a node's contents so that it can be correctly parsed as XML.
fn escape_encoded_node_contents(contents: &str) -> String {
    String::from(
        ESCAPE_ENCODED_REGEX.replace_all(contents, |caps: &Captures| {
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
        UNESCAPE_ENCODED_REGEX.replace_all(contents, |caps: &Captures| {
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

/// If a node is a <DOCUMENT>, parses its components into a SplitDocumentBeforeUpload.
/// If it's an <SEC-HEADER>, returns None
fn parse_doc(handle: &Rc<Node>) -> Option<SplitDocumentBeforeUpload> {
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

            let sequence = get_node_contents_as_int(&sequence_node);
            let doc_type = get_node_contents_as_str(&type_node);
            let maybe_description = match description_node {
                Some(d) => Some(get_node_contents_as_str(&d)),
                None => None,
            };
            let filename = get_node_contents_as_str(&filename_node);
            let text = get_text_node_children(&text_node);

            Some(SplitDocumentBeforeUpload {
                doc_type,
                sequence,
                filename,
                description: maybe_description,
                text,
                decoded_text: None, // we'll handle this later on
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

    //    #[test]
    //    fn test_local_file_1() {
    //        // Chosen at random
    //        let file_contents = include_str!("../../examples/10-Q/input/0001004434-17-000011.txt");
    //        let r = split_full_submission(file_contents);
    //        assert_eq!(83, r.len());
    //        //        write_parsed_docs_to_example_folder(r);
    //    }

    #[test]
    fn test_escape_encoded_node_contents() {
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
        let r = escape_encoded_node_contents(contents);
        assert_eq!(expected_contents, r);
    }

    #[test]
    fn test_escape_encoded_node_contents_excel() {
        let contents = r#"
        some
        other
        unimportant
        stuff
<TYPE>EXCEL
<SEQUENCE>6
<FILENAME>thing.xlsx
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
<TYPE>EXCEL
<SEQUENCE>6
<FILENAME>thing.xlsx
<DESCRIPTION>MATERIAL WEAKNESS REMEDIATION GRAPHIC
<TEXT>&lt;</TEXT>
    some
        nonsense
            after the fact
        "#;
        let r = escape_encoded_node_contents(contents);
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
