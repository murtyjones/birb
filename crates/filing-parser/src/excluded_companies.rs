// regex
use regex::Regex;

pub struct ExcludedCompany {
    pub cik: &'static str,
    pub name: &'static str,
    pub excludable_name: Regex,
}

fn build_excl_name(pat: &'static str) -> Regex {
    let insensitive = format!("(?i){}", pat);
    Regex::new(&insensitive).expect("Couldn't build income statement regex!")
}

lazy_static! {
    pub static ref EXCLUDED_COMPANIES: Vec<ExcludedCompany> = vec![
        ExcludedCompany {
            cik: "1003815",
            name: "BCTC IV ASSIGNOR CORP",
            excludable_name: build_excl_name("BCTC IV ASSIGNOR CORP"),
        },
        ExcludedCompany {
            cik: "1533218",
            name: "AMERICREDIT AUTOMOBILE RECEIVABLES TRUST 2011-5",
            excludable_name: build_excl_name("AMERICREDIT AUTOMOBILE RECEIVABLES TRUST"),
        },
        ExcludedCompany {
            cik: "1003509",
            name: "AMERICAN EXPRESS CREDIT ACCOUNT MASTER TRUST",
            excludable_name: build_excl_name("AMERICAN EXPRESS CREDIT ACCOUNT MASTER TRUST"),
        },
        ExcludedCompany {
            cik: "1644285",
            name: "GM FINANCIAL AUTOMOBILE LEASING TRUST 2015-2",
            excludable_name: build_excl_name("GM FINANCIAL AUTOMOBILE LEASING TRUST"),
        },
    ];
}
