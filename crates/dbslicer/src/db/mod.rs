use postgres::{Connection, TlsMode};
use postgres::rows::Rows;

pub fn get_connection<S>(host: S) -> Connection where S: Into<String> {
    Connection::connect(host.into(), TlsMode::None).unwrap()
}

pub fn get_companies(conn: &Connection) -> Rows {
    conn.query(
        r#"
            SELECT * FROM company
            ORDER BY created_at ASC
            LIMIT 100000
        "#, &[]
    ).expect("Couldn't get companies")
}
