#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use] extern crate rocket_contrib;


use std::env;
use std::ops::Deref;
use rocket::request::FromRequest;
use rocket::request::Request;
use rocket::request::State;
use rocket::request::Outcome;


pub struct Conn(pub PooledConnection<MongodbConnectionManager>);

/*
    create a connection pool of mongodb connections to allow a lot of users to modify db at same time.
*/
pub fn init_pool() -> Pool<MongodbConnectionManager> {
    dotenv().ok();
    let mongo_addr = env::var("MONGO_ADDR").expect("MONGO_ADDR must be set");
    let mongo_port = env::var("MONGO_PORT").expect("MONGO_PORT must be set");
    let db_name = env::var("DB_NAME").expect("DB_NAME env var must be set");
    let manager = MongodbConnectionManager::new(
        ConnectionOptionsBuilder::new()
            .with_host(&mongo_addr)
            .with_port(mongo_port.parse::<u16>().unwrap())
            .with_db(&db_name)
            .build(),
    );
    match Pool::builder().max_size(64).build(manager) {
        Ok(pool) => pool,
        Err(e) => panic!("Error: failed to create mongodb pool {}", e),
    }
}

/*
    When Conn is dereferencd, return the mongo connection.
*/
impl Deref for Conn {
    type Target = <r2d2_mongodb::MongodbConnectionManager as ManageConnection>::Connection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/*
    Create a implementation of FromRequest so Conn can be provided at every api endpoint
*/
impl<'a, 'r> FromRequest<'a, 'r> for Conn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Conn, ()> {
        let pool = request.guard::<State<Pool<MongodbConnectionManager>>>()?;
        match pool.get() {
            Ok(db) => Outcome::Success(Conn(db)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}


#[get("/hello")]
pub fn helloworld(
    conn: Conn,
) -> Json<Value> {
    let db = &conn;  // need to deref to get a mongo connection
    db.collections("hello").insert_one(doc!{ "name": "John" }, None).unwrap();
    Json(json!({ "status": "ok"}))
}


fn main() {

    // launch the app and mount all the api endpoints.
    rocket::ignite()
        .manage(init_pool())
        .mount("/", routes![helloworld)
    .launch();
}
