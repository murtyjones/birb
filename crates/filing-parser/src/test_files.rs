#[derive(Debug)]
pub struct TestableFiling {
    pub s3: &'static str,
    pub path: &'static str,
    pub table_element: &'static str,
}

pub fn get_files() -> Vec<TestableFiling> {
    return vec![
        TestableFiling {
            s3: "edgar/data/1000045/0001193125-18-037381.txt",
            path: "examples/10-Q/input/0001193125-18-037381.txt",
            table_element: "<table cellspacing=\"0\" cellpadding=\"0\" width=\"100%\" border=\"0\" align=\"center\" style=\"background-color: red;\" x-birb-income-statement-table=\"0\">"
        },
//        TestableFiling {
//            s3: "edgar/data/1000623/0001000623-17-000125.txt",
//            path: "examples/10-Q/input/0001000623-17-000125.txt",
//            table_element: "<table cellpadding=\"0\" cellspacing=\"0\" style=\"background-color: red;\" x-birb-income-statement-table=\"0\">"
//        },
//        TestableFiling {
//            s3: "edgar/data/1002037/0001437749-16-025027.txt",
//            path: "examples/10-Q/input/0001437749-16-025027.txt",
//            table_element: "<table id=\"TBL1155\" cellspacing=\"0\" cellpadding=\"0\" border=\"0\" style=\"background-color: red;\" x-birb-income-statement-table=\"0\">"
//        },
//        TestableFiling {
//            s3: "edgar/data/1004434/0001004434-17-000011.txt",
//            path: "examples/10-Q/input/0001004434-17-000011.txt",
//            table_element: "<table cellpadding=\"0\" cellspacing=\"0\" style=\"background-color: red;\" x-birb-income-statement-table=\"0\">"
//        },
//        TestableFiling {
//            s3: "edgar/data/1001463/0001185185-16-005721.txt",
//            path: "examples/10-Q/input/0001185185-16-005721.txt",
//            table_element: "<table id=\"z812e1f80f42848f48cfdf81d8682edfd\" cellspacing=\"0\" cellpadding=\"0\" style=\"background-color: red;\" x-birb-income-statement-table=\"0\">"
//        },
//        TestableFiling {
//            s3: "edgar/data/1012019/0001437749-16-036870.txt",
//            path: "examples/10-Q/input/0001437749-16-036870.txt",
//            table_element: "<table id=\"TBL2422\" cellspacing=\"0\" cellpadding=\"0\" border=\"0\" style=\"background-color: red;\" x-birb-income-statement-table=\"0\">"
//        },
//        TestableFiling {
//            s3: "edgar/data/1000045/0001193125-16-454777.txt",
//            path: "examples/10-Q/input/0001193125-16-454777.txt",
//            table_element: "<table cellspacing=\"0\" cellpadding=\"0\" width=\"100%\" border=\"0\" align=\"center\" style=\"background-color: red;\" x-birb-income-statement-table=\"0\">"
//        },
//        TestableFiling {
//            s3: "edgar/data/1000697/0001193125-17-160261.txt",
//            path: "examples/10-Q/input/0001193125-17-160261.txt",
//            table_element: "<table cellspacing=\"0\" cellpadding=\"0\" width=\"100%\" border=\"0\" align=\"center\" style=\"background-color: red;\" x-birb-income-statement-table=\"0\">"
//        },
//        TestableFiling {
//            s3: "edgar/data/1001288/0001001288-16-000069.txt",
//            path: "examples/10-Q/input/0001001288-16-000069.txt",
//            table_element: "<table cellspacing=\"0\" cellpadding=\"0\" style=\"background-color: red;\" x-birb-income-statement-table=\"0\">"
//        },
//        TestableFiling {
//            s3: "edgar/data/100625/0001493152-17-009297.txt",
//            path: "examples/10-Q/input/0001493152-17-009297.txt",
//            table_element: "<table cellpadding=\"0\" cellspacing=\"0\" style=\"background-color: red;\" x-birb-income-statement-table=\"0\">"
//        },
//        TestableFiling {
//            s3: "edgar/data/1015383/0001079973-17-000690.txt",
//            path: "examples/10-Q/input/0001079973-17-000690.txt",
//            table_element: "<table id=\"ze4e14be6186a48a58a40f9d2654a9a34\" cellspacing=\"0\" cellpadding=\"0\" style=\"background-color: red;\" x-birb-income-statement-table=\"0\">"
//        },
//        TestableFiling {
//            s3: "edgar/data/10254/0001564590-17-009385.txt",
//            path: "examples/10-Q/input/0001564590-17-009385.txt",
//            table_element: "<table border=\"0\" cellspacing=\"0\" cellpadding=\"0\" align=\"center\" style=\"background-color: red;\" x-birb-income-statement-table=\"0\">"
//        },
//        TestableFiling {
//            s3: "edgar/data/1027099/0001185185-16-005747.txt",
//            path: "examples/10-Q/input/0001185185-16-005747.txt",
//            table_element: "<table id=\"z1f37cd191a1742a59339b605e5b1593d\" cellspacing=\"0\" cellpadding=\"0\" style=\"background-color: red;\" x-birb-income-statement-table=\"0\">"
//        },
//        TestableFiling {
//            s3: "edgar/data/1029581/0001213900-16-018375.txt",
//            path: "examples/10-Q/input/0001213900-16-018375.txt",
//            table_element: "<table cellpadding=\"0\" cellspacing=\"0\" style=\"background-color: red;\" x-birb-income-statement-table=\"0\">"
//        },
//        TestableFiling { // this one is messy... two companies
//            s3: "edgar/data/1004980/0001004980-16-000073.txt",
//            path: "examples/10-Q/input/0001004980-16-000073.txt",
//            table_element: "<table id=\"6d37b329224548c18649e727b91c963c\" cellspacing=\"0\" cellpadding=\"0\" style=\"background-color: red;\" x-birb-income-statement-table=\"0\">"
//        },
//        TestableFiling {
//            s3: "edgar/data/1015780/0001015780-17-000075.txt",
//            path: "examples/10-Q/input/0001015780-17-000075.txt",
//            table_element: "<table cellpadding=\"0\" cellspacing=\"0\" style=\"background-color: red;\" x-birb-income-statement-table=\"0\">"
//        },
    ];
}
