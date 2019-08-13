use crate::excluded_companies::sec_header::EXCLUDED_COMPANIES;
use crate::excluded_companies::ExcludedCompany;
use crate::helpers::{bfs, bfs_no_base_case, bfs_skip_chillins};
use crate::processing_steps::table_accessor::TableAccessor;
use html5ever::rcdom::{Handle, Node, NodeData, RcDom};

#[derive(Debug, Fail, PartialEq)]
pub enum ProcessingError {
    #[fail(display = "No SEC-HEADER found: {}", cik)]
    NoSecHeaderFound { cik: String },
}

pub trait MetadataRemover: TableAccessor {
    fn probably_strip_metadata_nodes(&mut self) -> Result<(), ProcessingError> {
        // Process the filing
        let result = self.process();
        /*
         * if there are errors in finding expected tables, check
         * whether or not the filing contains the CIK of a company
         * that is known to not contain those tables. Some companies
         * don't include an income statement, for example. If the
         * filer isn't in this whitelist, return the errors.
         *
         * See: https://www.sec.gov/Archives/edgar/data/1003815/000100381516000011/b4assignorcorp121510k.htm
         */
        if let Err(e) = result {
            if self.excluable_filing().is_none() {
                return Err(e);
            }
        }

        let doc = self.get_doc();
        bfs_skip_chillins(doc, |n| self.strip_any_xbrl_node(&n));

        Ok(())
    }

    fn excluable_filing(&mut self) -> Option<&ExcludedCompany> {
        EXCLUDED_COMPANIES.iter().find(|&ex_company| {
            self.filing_contents().contains(ex_company.cik)
                || ex_company.excludable_name.is_match(self.filing_contents())
        })
    }

    fn process(&mut self) -> Result<(), ProcessingError> {
        let doc = self.get_doc();

        let stripped_header = bfs(doc, |n| self.maybe_strip_header_node(&n));

        if !stripped_header {
            return Err(ProcessingError::NoSecHeaderFound {
                cik: self.filing_key().clone(),
            });
        }

        Ok(())
    }

    fn maybe_strip_header_node(&mut self, handle: &Handle) -> bool {
        if self.node_is_sec_header(handle) {
            return true;
        };
        false
    }

    fn node_is_sec_header(&mut self, handle: &Handle) -> bool {
        if let NodeData::Element { ref name, .. } = handle.data {
            // Should be named <SEC-HEADER>
            if &name.local == "sec-header" || &name.local == "SEC-HEADER" {
                self.delete_node(handle);
                return true;
            }
        }
        false
    }

    fn strip_any_xbrl_node(&mut self, handle: &Handle) -> bool {
        if let NodeData::Element { ref name, .. } = handle.data {
            // Should be named <xbrl>
            if &name.local == "xbrl" || &name.local == "XBRL" {
                self.delete_node(handle);
                return true;
            }
        }
        false
    }
}