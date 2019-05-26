use rocket_contrib;

#[allow(missing_docs)]
#[catch(503)]
pub fn handler() -> rocket_contrib::json::JsonValue {
    json!({
        "status": "error",
        "reason": "Service not available. Is the DB up?",
    })
}
