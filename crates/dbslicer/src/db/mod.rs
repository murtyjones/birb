use postgres::{Connection, TlsMode};
use postgres::rows::{Rows,Row};
use models::{Company, Filing};
use rayon::prelude::*;

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

pub fn get_company_filings(conn: &Connection, company: &Company) -> Rows {
    conn.query(
        r#"
            SELECT * FROM filing
            WHERE short_cik = $1
            LIMIT 100000
        "#, &[&company.short_cik]
    ).expect("Couldn't get companies")
}

pub fn upsert_co_and_filings (conn: &Connection, c: &Company, filings: &Rows) {

    conn.execute(
        "
             INSERT INTO company
             (short_cik, company_name, created_at, updated_at)
             VALUES ($1, $2)
             ON CONFLICT (short_cik) DO NOTHING;
        ",
        &[&c.short_cik, &c.company_name, &c.created_at, &c.updated_at]
    ).expect("Couldn't insert company");

    let filing_upsert_stmt = trans
        .prepare(
            "
             INSERT INTO filing
             (id, company_short_cik, filing_name, filing_edgar_url, filing_quarter, filing_year, created_at, updated_at, collected)
             VALUES ($1, $2, $3, $4, $5);
             ",
        )
        .expect("Couldn't prepare filing upsert statement for execution");
    for row in filings.par_iter() {
        let f: Filing = row;
        filing_upsert_stmt.execute(
            &[&f.id, &f.short_cik, &f.filing_name, &f.filing_edgar_url, &f.filing_quarter, &f.filing_year, &f.created_at, &f.updated_at]
        ).expect("Couldn't perform update");
    }
}
