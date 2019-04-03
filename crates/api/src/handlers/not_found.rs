use rocket_contrib;

#[catch(404)]
pub fn handler() -> rocket_contrib::json::JsonValue {
    json!({
        "status": "error",
        "reason": "Resource not found.",
    })
}
