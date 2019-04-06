pub struct Database;

pub struct FakeCollection;

pub struct OrderedDocument;

impl FakeCollection {
    pub fn find_one(
        &self,
        filter: Option<OrderedDocument>,
        other_thing: Option<OrderedDocument>,
    ) -> Option<OrderedDocument> {
        Some(OrderedDocument)
    }
}

impl Database {
    pub fn collection(&self, collection: &'static str) -> FakeCollection {
        FakeCollection
    }
}
