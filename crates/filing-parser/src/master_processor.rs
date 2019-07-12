use html5ever::driver::parse_document;
use html5ever::rcdom::RcDom;
use html5ever::tendril::{SliceExt, TendrilSink};

use crate::processing_steps::table_type_identifier;

pub struct ParsedFiling {
    pub dom: RcDom,
}

#[derive(Debug, Fail, PartialEq)]
pub enum ParsingError {
    #[fail(display = "Failed to parse document for CIK: {}", cik)]
    FailedToParse { cik: String },
}

impl ParsedFiling {
    pub fn new(filing_contents: String) -> Result<ParsedFiling, Vec<ParsingError>> {
        let mut p_f = ParsedFiling {
            dom: parse_document(RcDom::default(), Default::default()).one(&*filing_contents),
        };

        // Return the processed document
        Ok(p_f)
    }
}
