pub mod db;

use aws;
use std::thread::sleep;
use models::Company;
use db::*;
use rayon::prelude::*;

pub fn get_prod_conn_string() -> String {
    let port = std::str::from_utf8(include_bytes!("../../../scripts/local_port")).unwrap().to_string();
    let uname = std::str::from_utf8(include_bytes!("../../../terraform/out/rds_db_username")).unwrap().to_string();
    let passwd = std::str::from_utf8(include_bytes!("../../../terraform/out/rds_db_password")).unwrap().to_string();
    let db_name = std::str::from_utf8(include_bytes!("../../../terraform/out/rds_db_name")).unwrap().to_string();

    format!("postgres://{}:{}@localhost:{}/{}", uname, passwd, port, db_name)
}

fn main() {
//    sleep(std::time::Duration::from_secs(3));
    let prod_conn_string = get_prod_conn_string();
    let production_connection = get_connection(prod_conn_string);
    let local_connection = get_connection("postgres://postgres:develop@localhost:5432/postgres");
    truncate_local_companies(&local_connection);
    let companies = get_companies(&production_connection);

    let trans = local_connection.transaction().expect("Couldn't begin transaction");
    for (i, company) in companies.iter().enumerate() {
        println!("Inserting record {}", i);
        let company_filings = get_company_filings(&production_connection, &company);
        upsert_co_and_filings(&trans, &company, &company_filings);
    }
    trans.commit().expect("Couldn't commit transaction");
}
