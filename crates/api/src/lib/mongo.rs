#[allow(unused)]
use mongodb::db::ThreadedDatabase;
use mongodb::ordered::OrderedDocument;

#[cfg(not(test))]
pub mod db {
    pub use mongodb::db::Database;
}

#[cfg(test)]
pub mod db {
    pub use crate::mocks::mongodb::Database;
}

pub struct Mongo;

impl Mongo {
    pub fn find_one(
        conn: &db::Database,
        collection: &'static str,
        filter: Option<OrderedDocument>,
    ) -> Option<OrderedDocument> {
        conn.collection(collection).find_one(filter, None).unwrap()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn find_one_unwraps() {
        assert_eq!(1, 1)
    }
}
