use bson;
use bson::Array;
use mongodb::db::Database;
use mongodb::db::ThreadedDatabase;
use mongodb::ordered::OrderedDocument;

#[derive(Debug)]
pub struct Model {
    pub cik: String,
    pub names: Array,
}

impl Model {
    pub fn find_one_by_cik(conn: &Database, cik: String) -> Option<OrderedDocument> {
        conn.collection("company")
            .find_one(Some(doc! { "cik" => cik }), None)
            .unwrap()
    }
}
