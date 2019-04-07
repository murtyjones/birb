#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use(bson, doc)]
extern crate bson;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate mongodb;
extern crate serde_json;

mod handlers;
mod lib;
mod meta;
mod mocks;
mod models;

#[database("mongo_datastore")]
pub struct DbConnection(mongodb::db::Database);

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
