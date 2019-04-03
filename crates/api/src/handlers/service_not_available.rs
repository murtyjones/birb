use rocket_contrib;

#[catch(503)]
pub fn handler() -> rocket_contrib::json::JsonValue {
    json!({
        "status": "error",
        "reason": "Service not available. Is the DB up?",
    })
}
