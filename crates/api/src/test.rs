use super::rocket;
use rocket::http::{ContentType, Status};
use rocket::local::Client;

#[test]
fn bad_get() {
    let client = Client::new(rocket()).unwrap();
    let res = client
        .get("/doesnotexist")
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(res.status(), Status::NotFound);
}

#[test]
fn get_tsla() {
    let client = Client::new(rocket()).unwrap();
    let mut res = client
        .get("/company/0001318605")
        .header(ContentType::JSON)
        .dispatch();
    let body = res.body_string().unwrap();
    assert_eq!(res.status(), Status::Ok);
    assert!(body.contains("TESLA MOTORS INC"));
    assert!(body.contains("0001318605"));
}
