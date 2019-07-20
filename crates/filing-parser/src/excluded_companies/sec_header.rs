// regex
use crate::excluded_companies::{build_excl_name, ExcludedCompany};
use regex::Regex;

lazy_static! {
    pub static ref EXCLUDED_COMPANIES: Vec<ExcludedCompany> = vec![];
}

#[cfg(test)]
mod test {
    use super::*;
}
