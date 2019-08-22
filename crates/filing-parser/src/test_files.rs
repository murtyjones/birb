use bb::bb_filesystem::bb_root_dir;

#[derive(Debug)]
pub struct TestableFiling {
    pub s3: &'static str,
    pub path: String,
}

fn make_path(partial: &'static str) -> String {
    format!(
        "{}/crates/filing-parser/{}",
        bb_root_dir().display(),
        partial
    )
    .to_string()
}

pub fn get_files() -> Vec<TestableFiling> {
    return vec![
        TestableFiling {
            s3: "edgar/data/1000045/0001193125-18-037381.txt.gz",
            path: make_path("examples/10-Q/input/0001193125-18-037381"),
        },
        TestableFiling {
            s3: "edgar/data/1000623/0001000623-17-000125.txt.gz",
            path: make_path("examples/10-Q/input/0001000623-17-000125"),
        },
        TestableFiling {
            s3: "edgar/data/1002037/0001437749-16-025027.txt.gz",
            path: make_path("examples/10-Q/input/0001437749-16-025027"),
        },
        TestableFiling {
            s3: "edgar/data/1004434/0001004434-17-000011.txt.gz",
            path: make_path("examples/10-Q/input/0001004434-17-000011"),
        },
        TestableFiling {
            s3: "edgar/data/1001463/0001185185-16-005721.txt.gz",
            path: make_path("examples/10-Q/input/0001185185-16-005721"),
        },
        TestableFiling {
            s3: "edgar/data/1012019/0001437749-16-036870.txt.gz",
            path: make_path("examples/10-Q/input/0001437749-16-036870"),
        },
        TestableFiling {
            s3: "edgar/data/1000045/0001193125-16-454777.txt.gz",
            path: make_path("examples/10-Q/input/0001193125-16-454777"),
        },
        TestableFiling {
            s3: "edgar/data/1000697/0001193125-17-160261.txt.gz",
            path: make_path("examples/10-Q/input/0001193125-17-160261"),
        },
        TestableFiling {
            s3: "edgar/data/1001288/0001001288-16-000069.txt.gz",
            path: make_path("examples/10-Q/input/0001001288-16-000069"),
        },
        TestableFiling {
            s3: "edgar/data/100625/0001493152-17-009297.txt.gz",
            path: make_path("examples/10-Q/input/0001493152-17-009297"),
        },
        TestableFiling {
            s3: "edgar/data/1015383/0001079973-17-000690.txt.gz",
            path: make_path("examples/10-Q/input/0001079973-17-000690"),
        },
        TestableFiling {
            s3: "edgar/data/10254/0001564590-17-009385.txt.gz",
            path: make_path("examples/10-Q/input/0001564590-17-009385"),
        },
        TestableFiling {
            s3: "edgar/data/1027099/0001185185-16-005747.txt.gz",
            path: make_path("examples/10-Q/input/0001185185-16-005747"),
        },
        TestableFiling {
            s3: "edgar/data/1029581/0001213900-16-018375.txt.gz",
            path: make_path("examples/10-Q/input/0001213900-16-018375"),
        },
        TestableFiling {
            // this one is messy... two companies
            s3: "edgar/data/1004980/0001004980-16-000073.txt.gz",
            path: make_path("examples/10-Q/input/0001004980-16-000073"),
        },
        TestableFiling {
            s3: "edgar/data/1015780/0001015780-17-000075.txt.gz",
            path: make_path("examples/10-Q/input/0001015780-17-000075"),
        },
        TestableFiling {
            s3: "edgar/data/1000694/0001144204-16-084770.txt.gz",
            path: make_path("examples/10-Q/input/0001144204-16-084770"),
        },
    ];
}
