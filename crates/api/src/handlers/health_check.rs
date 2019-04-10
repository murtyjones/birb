#[get("/")]
pub fn get() -> &'static str {
    "OK!"
}
