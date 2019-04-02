#[macro_use]
use bson;
use mongodb::db::Database;
use mongodb::ordered::OrderedDocument;

#[derive(Debug)]
pub struct Model {
    pub cik: String,
}

impl Model {
    pub fn find_one_by_cik(conn: &Database, cik: String) -> Option<OrderedDocument> {
        conn.collection("company")
            .find_one(Some(doc! { "CIK" => cik }), None)
            .unwrap()
    }
}
