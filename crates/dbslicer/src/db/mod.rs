use postgres::{Connection, TlsMode};
use postgres::rows::{Rows,Row};
use models::Company;

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

fn get_co_model(companies: &Rows) -> Vec<Company> {
    let mut binds = Vec::new();
    for row in companies {
        binds.push(Company {
            short_cik: row.get("short_cik"),
            company_name: row.get("company_name"),
        });
    }
    binds
}

pub fn get_filings(conn: &Connection, companies: &Rows) -> Rows {
    let companies = get_co_model(companies);
    let short_ciks = companies.iter().map(|c: &Company|
        c.short_cik
    );
    conn.query(
        r#"
            SELECT * FROM company
            ORDER BY created_at ASC
            LIMIT 100000
        "#, &[]
    ).expect("Couldn't get companies")
}

pub fn get_filing_types(conn: &Connection) -> Rows {
    conn.query(
        r#"
            SELECT * FROM filing_type
        "#, &[]
    ).expect("Couldn't get companies")
}
