use postgres::{Connection, TlsMode};
use postgres::rows::Rows;

pub fn get_connection<S>(host: S) -> Connection where S: Into<String> {
    Connection::connect(host.into(), TlsMode::None).unwrap()
}

pub fn get_companies(conn: &Connection) -> Rows {
    conn.query("SELECT * FROM companies LIMIT 100", &[]).expect("Couldn't get companies")
}
