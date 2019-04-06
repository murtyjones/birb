pub struct FakeCollection;

pub trait Database {
    fn collection() -> FakeCollection;
}
