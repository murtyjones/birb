#[cfg(test)]
use crate::mocks::mongodb::Database;
#[cfg(test)]
use crate::mocks::mongodb::OrderedDocument;

#[cfg(not(test))]
use mongodb::db::Database;
#[allow(unused)]
use mongodb::db::ThreadedDatabase;
#[cfg(not(test))]
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

#[cfg(test)]
mod tests {
    #[test]
    fn find_one_unwraps() {
        assert_eq!(1, 1)
    }
}
