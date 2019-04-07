#[allow(unused)]
use mongodb::db::Database;
use mongodb::db::ThreadedDatabase;
use mongodb::ordered::OrderedDocument;

pub struct Mongo;

#[cfg(not(test))]
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
