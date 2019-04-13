#[allow(unused)] use mongodb::db::Database;
use mongodb::db::ThreadedDatabase;
use mongodb::ordered::OrderedDocument;

/// handles mongo interactions
pub struct Mongo;

impl Mongo {
    /// Find a document in a given collection
    pub fn find_one(
        conn: &Database,
        collection: &'static str,
        filter: Option<OrderedDocument>,
    ) -> Option<OrderedDocument> {
        conn.collection(collection).find_one(filter, None).unwrap()
    }
}
