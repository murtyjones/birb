#[cfg(test)]
use crate::mocks::mongodb::Database;
#[cfg(not(test))]
use mongodb::db::Database;
use mongodb::db::ThreadedDatabase;
use mongodb::ordered::OrderedDocument;

pub struct Mongo;

impl Mongo {
    pub fn find_one(
        conn: &Database,
        collection: &'static str,
        filter: Option<OrderedDocument>,
    ) -> Option<OrderedDocument> {
        conn.collection(collection).find_one(filter, None).unwrap()
    }
}
