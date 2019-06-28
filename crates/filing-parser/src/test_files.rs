pub struct TestableFiling {
    pub path: String,
    pub header_inner_html: String,
}

lazy_static! {
    pub static ref FILES: Vec<TestableFiling> = vec![
        TestableFiling {
            path: String::from("./examples/10-Q/input/0001193125-18-037381.txt"),
            header_inner_html: String::from("Consolidated Statements of Income (Loss) "),
        },
        TestableFiling {
            path: String::from("./examples/10-Q/input/0001000623-17-000125.txt"),
            header_inner_html: String::from("CONDENSED CONSOLIDATED STATEMENTS OF INCOME"),
        },
        TestableFiling {
            path: String::from("./examples/10-Q/input/0001437749-16-025027.txt"),
            header_inner_html: String::from(
                "CONDENSED CONSOLIDATED STATEMENTS OF OPERATIONS AND COMPREHENSIVE LOSS"
            ),
        },
        TestableFiling {
            path: String::from("./examples/10-Q/input/0001004434-17-000011.txt"),
            header_inner_html: String::from("CONSOLIDATED STATEMENTS OF INCOME"),
        },
        TestableFiling {
            path: String::from("./examples/10-Q/input/0001185185-16-005721.txt"),
            header_inner_html: String::from("CONSOLIDATED STATEMENTS OF OPERATIONS"),
        },
        TestableFiling {
            path: String::from("./examples/10-Q/input/0001437749-16-036870.txt"),
            header_inner_html: String::from(
                "CONSOLIDATED STATEMENTS OF INCOME AND COMPREHENSIVE INCOME"
            ),
        },
        TestableFiling {
            path: String::from("./examples/10-Q/input/0001193125-16-454777.txt"),
            header_inner_html: String::from("Consolidated Statements of Income "),
        },
        TestableFiling {
            path: String::from("./examples/10-Q/input/0001193125-17-160261.txt"),
            header_inner_html: String::from("CONSOLIDATED STATEMENTS OF OPERATIONS "),
        },
        TestableFiling {
            path: String::from("./examples/10-Q/input/0001001288-16-000069.txt"),
            header_inner_html: String::from("CONSOLIDATED CONDENSED STATEMENTS OF EARNINGS"),
        },
    ];
}
