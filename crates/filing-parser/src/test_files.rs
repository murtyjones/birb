#[derive(Debug)]
pub struct TestableFiling {
    pub s3: String,
    pub path: String,
    pub header_inner_html: String,
}

lazy_static! {
    pub static ref FILES: Vec<TestableFiling> = vec![
        TestableFiling {
            s3: String::from("edgar/data/1000045/0001193125-18-037381.txt"),
            path: String::from("./examples/10-Q/input/0001193125-18-037381.txt"),
            header_inner_html: String::from("Consolidated Statements of Income (Loss) "),
        },
        TestableFiling {
            s3: String::from("edgar/data/1000623/0001000623-17-000125.txt"),
            path: String::from("./examples/10-Q/input/0001000623-17-000125.txt"),
            header_inner_html: String::from("CONDENSED CONSOLIDATED STATEMENTS OF INCOME"),
        },
        TestableFiling {
            s3: String::from("edgar/data/1002037/0001437749-16-025027.txt"),
            path: String::from("./examples/10-Q/input/0001437749-16-025027.txt"),
            header_inner_html: String::from(
                "CONDENSED CONSOLIDATED STATEMENTS OF OPERATIONS AND COMPREHENSIVE LOSS"
            ),
        },
        TestableFiling {
            s3: String::from("edgar/data/1004434/0001004434-17-000011.txt"),
            path: String::from("./examples/10-Q/input/0001004434-17-000011.txt"),
            header_inner_html: String::from("CONSOLIDATED STATEMENTS OF INCOME"),
        },
        TestableFiling {
            s3: String::from("edgar/data/1001463/0001185185-16-005721.txt"),
            path: String::from("./examples/10-Q/input/0001185185-16-005721.txt"),
            header_inner_html: String::from("CONSOLIDATED STATEMENTS OF OPERATIONS"),
        },
        TestableFiling {
            s3: String::from("edgar/data/1012019/0001437749-16-036870.txt"),
            path: String::from("./examples/10-Q/input/0001437749-16-036870.txt"),
            header_inner_html: String::from(
                "CONSOLIDATED STATEMENTS OF INCOME AND COMPREHENSIVE INCOME"
            ),
        },
        TestableFiling {
            s3: String::from("edgar/data/1000045/0001193125-16-454777.txt"),
            path: String::from("./examples/10-Q/input/0001193125-16-454777.txt"),
            header_inner_html: String::from("Consolidated Statements of Income "),
        },
        TestableFiling {
            s3: String::from("edgar/data/1000697/0001193125-17-160261.txt"),
            path: String::from("./examples/10-Q/input/0001193125-17-160261.txt"),
            header_inner_html: String::from("CONSOLIDATED STATEMENTS OF OPERATIONS "),
        },
        TestableFiling {
            s3: String::from("edgar/data/1001288/0001001288-16-000069.txt"),
            path: String::from("./examples/10-Q/input/0001001288-16-000069.txt"),
            header_inner_html: String::from("CONSOLIDATED CONDENSED STATEMENTS OF EARNINGS"),
        },
        TestableFiling {
            s3: String::from("edgar/data/100625/0001493152-17-009297.txt"),
            path: String::from("./examples/10-Q/input/0001493152-17-009297.txt"),
            header_inner_html: String::from("Condensed\nStatements of Operations"),
        },
    ];
}
