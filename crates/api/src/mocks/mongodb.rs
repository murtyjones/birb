use mongodb::ordered::OrderedDocument;
use std::io::Error;
use std::result::Result;

#[cfg(test)]
pub struct Database;

pub struct FakeCollection;

impl FakeCollection {
    pub fn find_one(
        &self,
        filter: Option<OrderedDocument>,
        other_thing: Option<OrderedDocument>,
    ) -> Result<Option<OrderedDocument>, Error> {
        let fake_doc: OrderedDocument = OrderedDocument::default();
        Result::Ok(Some(fake_doc))
    }
}

#[cfg(test)]
impl Database {
    pub fn collection(&self, collection: &'static str) -> FakeCollection {
        FakeCollection
    }
}
