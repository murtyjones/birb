use postgres::{Connection, TlsMode};

pub fn get_connection(host: &'static str) {
    let conn = Connection::connect(host, TlsMode::None).unwrap();
}
