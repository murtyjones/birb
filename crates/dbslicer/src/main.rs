pub mod db;

use aws;
use db::*;

pub fn get_prod_conn_string() -> String {
    let port = std::str::from_utf8(include_bytes!("../../../scripts/local_port")).unwrap().to_string();
    let uname = std::str::from_utf8(include_bytes!("../../../terraform/out/rds_db_username")).unwrap().to_string();
    let passwd = std::str::from_utf8(include_bytes!("../../../terraform/out/rds_db_password")).unwrap().to_string();
    let db_name = std::str::from_utf8(include_bytes!("../../../terraform/out/rds_db_name")).unwrap().to_string();

    format!("postgres://{}:{}@127.0.0.1:{}/{}", uname, passwd, port, db_name)
}

fn main() {
    let prod_conn_string = get_prod_conn_string();
    let production_connection = get_connection(prod_conn_string);
    let local_connection = get_connection("postgres://postgres:develop@db:5432/postgres");
    let companies = get_companies(&production_connection);
    for row in companies.iter() {
        panic!("Found company {:?}", row);
    }
}
