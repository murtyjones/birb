extern crate filing_parser;
#[macro_use]
extern crate lazy_static;

use filing_parser::helpers::{bfs_find_node, tendril_to_string, write_to_file};
use filing_parser::uu;
use regex::{Captures, Regex, RegexBuilder};
use std::rc::Rc;
use xml5ever::driver::parse_document;
use xml5ever::rcdom::{Handle, Node, NodeData, RcDom};
use xml5ever::serialize::serialize;
use xml5ever::tendril::TendrilSink;


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

/// For now, iterates through our list of full submission ttext files and
/// splits each into its seperate parts, then writes to disk.
fn main() {
    let test_files: Vec<&'static str> = vec![
        include_str!("../../examples/10-Q/input/0001000623-17-000125.txt"),
//        include_str!("../../examples/10-Q/input/0001001288-16-000069.txt"),
//        include_str!("../../examples/10-Q/input/0001004434-17-000011.txt"),
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
        let sec_document_node = &document.children.borrow()[0];
        assert_sec_document_node_is_sec_document(sec_document_node);
        let parsed_documents = split_document_into_documents(sec_document_node);
        let unescaped_parsed_documents = unescape_parsed_documents(parsed_documents);

        write_parsed_docs_to_example_folder(unescaped_parsed_documents);
    }
}

/// Applies the unescape logic to all the GRAPHIC documents, then returns all parsed documents.
fn unescape_parsed_documents(parsed_docs: Vec<ParsedDocument>) -> Vec<ParsedDocument> {
    let mut escaped_parsed_docs = vec![];
    for mut doc in parsed_docs {
        if doc.type_ == "GRAPHIC" {
            doc.text = String::from(unescape_node_contents(&doc.text));
            doc.text = String::from(fix_broken_graphic_node_contents(&doc.text));
        }
        escaped_parsed_docs.push(doc);
    }
    escaped_parsed_docs
}

/// Takes the <SEC-DOCUMENT> node of the document and tries to parse its children
/// into seperate documents. Most of its children are <DOCUMENT> nodes, which
/// should be parsable. Some however are things like <SEC-HEADER>, which is not
/// a document. Hence the filter_map.
fn split_document_into_documents(sec_document_node: &Handle) -> Vec<ParsedDocument> {
    sec_document_node
        .children
        .borrow()
        .iter()
        // Filter to only the parsed documents that succeeded:
        .filter_map(|node| parse_doc(node))
        .collect::<Vec<ParsedDocument>>()
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

/// If the GRAPHIC text content ends like this:
/// ```
/// 45NQ8LF7-GD6;5NU:MFW=@@T( #L!
///
/// end
/// ```
///
/// It should end like this:
/// ```
/// 45NQ8LF7-GD6;5NU:MFW=@@T( #L!
/// `
/// end
/// ```
fn fix_broken_graphic_node_contents(contents: &str) -> String {
    contents.replace("\n\nend", "\n`\nend")
}

/// Converts docs' contents to bytes (which may include uudecoding)
/// and writes them to an example folder locally.
fn write_parsed_docs_to_example_folder(parsed_documents: Vec<ParsedDocument>) {
    for doc in parsed_documents {
        let mut contents_for_file = doc.text.as_bytes().to_owned();
        if doc.type_ == "GRAPHIC" {
            //            panic!("{:?}", doc.text);
            contents_for_file = uu::decode_uu(&*doc.text)
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

/// If a node is a <DOCUMENT>, parses its components into a ParsedDocument.
/// If it's an <SEC-HEADER>, returns None
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
            let text_contents = get_text_node_children(&text_node);

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
  #[test]
    use super::{escape_graphic_node_contents,unescape_node_contents,fix_broken_graphic_node_contents};

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

    #[test]
    fn test_fix_broken_graphic_node_contents() {
        let contents = r#"=HHH **** "BBB@ HHHH **** "BBB@ HHHH _]D!

end"#;
        let result = fix_broken_graphic_node_contents(contents);

        assert_eq!(
            r#"=HHH **** "BBB@ HHHH **** "BBB@ HHHH _]D!
`
end"#,
            result
        )
    }
}
