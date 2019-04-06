pub struct Database;

pub struct FakeCollection;

pub trait ThreadedDatabase {
    fn collection(&self) -> FakeCollection;
}

impl ThreadedDatabase for Database {
    fn collection(&self) -> FakeCollection {
        FakeCollection
    }
}
