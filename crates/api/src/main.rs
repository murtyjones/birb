#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use(bson, doc)]
extern crate bson;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use lib::mongo::db::Database;
use rocket_contrib::databases::{r2d2_mongodb, DatabaseConfig, DbError, Poolable};

mod handlers;
mod lib;
mod meta;
mod mocks;
mod models;

#[cfg(test)]
#[cfg(feature = "mongodb_pool")]
impl Poolable for Database {
    type Manager = r2d2_mongodb::MongodbConnectionManager;
    type Error = DbError<mongodb::Error>;

    fn pool(config: DatabaseConfig) -> Result<r2d2::Pool<Self::Manager>, Self::Error> {
        let manager = r2d2_mongodb::MongodbConnectionManager::new_with_uri(config.url)
            .map_err(DbError::Custom)?;
        r2d2::Pool::builder()
            .max_size(config.pool_size)
            .build(manager)
            .map_err(DbError::PoolError)
    }
}

#[database("mongo_datastore")]
pub struct DbConnection(Database);

fn rocket() -> rocket::Rocket {
    return rocket::ignite()
        .attach(DbConnection::fairing())
        .mount("/", routes![handlers::company::get])
        .register(catchers![
            handlers::not_found::handler,
            handlers::service_not_available::handler
        ]);
}

fn main() {
    rocket().launch();
}

#[cfg(test)]
mod test {
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

}
