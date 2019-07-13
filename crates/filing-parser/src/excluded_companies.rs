pub struct ExcludedCompany {
    pub cik: &'static str,
    pub name: &'static str,
}

lazy_static! {
    pub static ref EXCLUDED_COMPANIES: Vec<ExcludedCompany> = vec![
        ExcludedCompany {
            cik: "1003815",
            name: "BCTC IV ASSIGNOR CORP",
        },
        ExcludedCompany {
            cik: "1533218",
            name: "AMERICREDIT AUTOMOBILE RECEIVABLES TRUST 2011-5",
        },
        ExcludedCompany {
            cik: "1003509",
            name: "AMERICAN EXPRESS CREDIT ACCOUNT MASTER TRUST",
        },
    ];
}
