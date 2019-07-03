#[derive(PartialEq, Debug)]
pub enum MatchType {
    Regex,
    Attribute,
}

#[derive(Debug)]
pub struct TestableFiling {
    pub s3: &'static str,
    pub path: &'static str,
    pub header_inner_html: Option<&'static str>,
    pub match_type: MatchType,
}

pub fn get_files() -> Vec<TestableFiling> {
    return vec![
        TestableFiling {
            s3: "edgar/data/1000045/0001193125-18-037381.txt",
            path: "examples/10-Q/input/0001193125-18-037381.txt",
            header_inner_html: Some("Consolidated Statements of Income (Loss) "),
            match_type: MatchType::Regex,
        },
        TestableFiling {
            s3: "edgar/data/1000623/0001000623-17-000125.txt",
            path: "examples/10-Q/input/0001000623-17-000125.txt",
            header_inner_html: Some("CONDENSED CONSOLIDATED STATEMENTS OF INCOME"),
            match_type: MatchType::Regex,
        },
        TestableFiling {
            s3: "edgar/data/1002037/0001437749-16-025027.txt",
            path: "examples/10-Q/input/0001437749-16-025027.txt",
            header_inner_html: Some(
                "CONDENSED CONSOLIDATED STATEMENTS OF OPERATIONS AND COMPREHENSIVE LOSS",
            ),
            match_type: MatchType::Regex,
        },
        TestableFiling {
            s3: "edgar/data/1004434/0001004434-17-000011.txt",
            path: "examples/10-Q/input/0001004434-17-000011.txt",
            header_inner_html: Some("CONSOLIDATED STATEMENTS OF INCOME"),
            match_type: MatchType::Regex,
        },
        TestableFiling {
            s3: "edgar/data/1001463/0001185185-16-005721.txt",
            path: "examples/10-Q/input/0001185185-16-005721.txt",
            header_inner_html: Some("CONSOLIDATED STATEMENTS OF OPERATIONS"),
            match_type: MatchType::Regex,
        },
        TestableFiling {
            s3: "edgar/data/1012019/0001437749-16-036870.txt",
            path: "examples/10-Q/input/0001437749-16-036870.txt",
            header_inner_html: Some("CONSOLIDATED STATEMENTS OF INCOME AND COMPREHENSIVE INCOME"),
            match_type: MatchType::Regex,
        },
        TestableFiling {
            s3: "edgar/data/1000045/0001193125-16-454777.txt",
            path: "examples/10-Q/input/0001193125-16-454777.txt",
            header_inner_html: Some("Consolidated Statements of Income "),
            match_type: MatchType::Regex,
        },
        TestableFiling {
            s3: "edgar/data/1000697/0001193125-17-160261.txt",
            path: "examples/10-Q/input/0001193125-17-160261.txt",
            header_inner_html: Some("CONSOLIDATED STATEMENTS OF OPERATIONS "),
            match_type: MatchType::Regex,
        },
        TestableFiling {
            s3: "edgar/data/1001288/0001001288-16-000069.txt",
            path: "examples/10-Q/input/0001001288-16-000069.txt",
            header_inner_html: Some("CONSOLIDATED CONDENSED STATEMENTS OF EARNINGS"),
            match_type: MatchType::Regex,
        },
        TestableFiling {
            s3: "edgar/data/100625/0001493152-17-009297.txt",
            path: "examples/10-Q/input/0001493152-17-009297.txt",
            header_inner_html: Some("Condensed\nStatements of Operations"),
            match_type: MatchType::Regex,
        },
        TestableFiling {
            s3: "edgar/data/1015383/0001079973-17-000690.txt",
            path: "examples/10-Q/input/0001079973-17-000690.txt",
            header_inner_html: Some("STATEMENT OF OPERATIONS"),
            match_type: MatchType::Regex,
        },
        TestableFiling {
            s3: "edgar/data/10254/0001564590-17-009385.txt",
            path: "examples/10-Q/input/0001564590-17-009385.txt",
            header_inner_html: None,
            match_type: MatchType::Attribute,
        },
        TestableFiling {
            s3: "edgar/data/1027099/0001185185-16-005747.txt",
            path: "examples/10-Q/input/0001185185-16-005747.txt",
            header_inner_html: None,
            match_type: MatchType::Attribute,
        },
        TestableFiling {
            s3: "edgar/data/1029581/0001213900-16-018375.txt",
            path: "examples/10-Q/input/0001213900-16-018375.txt",
            header_inner_html: Some("CONDENSED\nCONSOLIDATED STATEMENTS OF OPERATIONS"),
            match_type: MatchType::Regex,
        },
    ];
}
