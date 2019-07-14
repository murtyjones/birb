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
        ExcludedCompany {
            cik: "1059376",
            name: "Corporate Bond-Backed Certificates, Series 1998-CAT-1 Trust",
            excludable_name: build_excl_name(r"Corporate Bond-Backed Certificates, Series (19[5-9]\d|20[0-4]\d|2050)-CAT-1 Trust"),
        },
        ExcludedCompany {
            cik: "1059377",
            name: "CORPORATE BOND BACKED CERT TR SER 1998-ADM 1",
            excludable_name: build_excl_name(r"CORPORATE BOND BACKED CERT TR SER (19[5-9]\d|20[0-4]\d|2050)-ADM 1"),
        },
        ExcludedCompany {
            cik: "1059378",
            name: "CORPORATE BOND BACKED CERT TR SER 1998-NSC 1",
            excludable_name: build_excl_name(r"CORPORATE BOND BACKED CERT TR SER (19[5-9]\d|20[0-4]\d|2050)-NSC 1"),
        },
    ];
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cat1_trust_regex() {
        let company = &EXCLUDED_COMPANIES[4]; // 1059376
        let result = company.excludable_name.is_match(company.name);
        assert!(result);
    }

    #[test]
    fn test_adm1_trust_regex() {
        let company = &EXCLUDED_COMPANIES[5]; // 1059377
        let result = company.excludable_name.is_match(company.name);
        assert!(result);
    }
}
