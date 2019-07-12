pub struct ExcludedCompany {
    pub cik: &'static str,
    pub name: &'static str,
}

lazy_static! {
    pub static ref EXCLUDED_COMPANIES: Vec<ExcludedCompany> = vec![ExcludedCompany {
        cik: "1003815",
        name: "BCTC IV ASSIGNOR CORP",
    }];
}
