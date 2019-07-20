// regex
use crate::excluded_companies::{build_excl_name, ExcludedCompany};
use regex::Regex;

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
        ExcludedCompany {
            cik: "1092367",
            name: "SYCAMORE NETWORKS, INC.",
            excludable_name: build_excl_name(r"SYCAMORE NETWORKS, INC."),
        },
        ExcludedCompany {
            cik: "1080448",
            name: "PATRIOT GOLD CORP.",
            excludable_name: build_excl_name(r"PATRIOT GOLD CORP."),
        },
        ExcludedCompany {
            cik: "1110795",
            name: "METLIFE POLICYHOLDER TRUST",
            excludable_name: build_excl_name(r"METLIFE POLICYHOLDER TRUST"),
        },
        ExcludedCompany {
            cik: "921864",
            name: "CITIBANK CREDIT CARD MASTER TRUST I",
            excludable_name: build_excl_name(r"CITIBANK CREDIT CARD MASTER TRUST (I|II|III|IV|V|VI|VII|VIII|IX|X)"),
        },
        ExcludedCompany {
            cik: "1128250",
            name: "BA CREDIT CARD TRUST",
            excludable_name: build_excl_name(r"BA Credit Card Trust"),
        },
        ExcludedCompany {
            cik: "1128383",
            name: "NEVADA CLASSIC THOROUGHBREDS INC",
            excludable_name: build_excl_name(r"NEVADA CLASSIC THOROUGHBREDS INC"),
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
