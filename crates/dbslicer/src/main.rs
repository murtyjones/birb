use utils::get_connection;

pub mod db;
use db::*;

pub fn get_prod_read_only_conn_string() -> String {
    let port = format!("{}/../../scripts/local_port", env!("CARGO_MANIFEST_DIR"));
    let port = std::path::Path::new(&port);
    let username = format!(
        "{}/../../scripts/terraform/out/rds_db_username",
        env!("CARGO_MANIFEST_DIR")
    );
    let username = std::path::Path::new(&username);
    let password = format!(
        "{}/../../scripts/terraform/out/rds_db_password",
        env!("CARGO_MANIFEST_DIR")
    );
    let password = std::path::Path::new(&password);
    let db_name = format!(
        "{}/../../scripts/terraform/out/rds_db_name",
        env!("CARGO_MANIFEST_DIR")
    );
    let db_name = std::path::Path::new(&db_name);

    if username.is_file() && password.is_file() && db_name.is_file() {
        let port = std::fs::read_to_string(port).unwrap();
        let username = std::fs::read_to_string(username).unwrap();
        let password = std::fs::read_to_string(password).unwrap();
        let db_name = std::fs::read_to_string(db_name).unwrap();
        return format!(
            "postgres://{}:{}@localhost:{}/{}",
            username, password, port, db_name
        );
    }
    panic!("No prod conn string available!")
}

fn main() {
    //    sleep(std::time::Duration::from_secs(3));
    let prod_conn_string = get_prod_read_only_conn_string();
    let production_connection = get_connection(prod_conn_string);
    let local_connection = get_connection("postgres://postgres:develop@localhost:5432/postgres");
    truncate_local_companies(&local_connection);
    let companies_and_filings = get_companies_and_filings(&production_connection);

    let trans = local_connection
        .transaction()
        .expect("Couldn't begin transaction");
    for (i, company_with_filing) in companies_and_filings.iter().enumerate() {
        println!("Inserting record {}", i);
        upsert_company_and_filings(&trans, &company_with_filing);
    }
    trans.commit().expect("Couldn't commit transaction");
}
