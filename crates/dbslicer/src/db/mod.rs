use postgres::{Connection, TlsMode};
use postgres::rows::{Rows,Row};
use models::{Company, Filing};
use rayon::prelude::*;

pub fn get_connection<S>(host: S) -> Connection where S: Into<String> {
    Connection::connect(host.into(), TlsMode::None).unwrap()
}

pub fn get_companies(conn: &Connection) -> Vec<Company> {
    // Get a small slice of companies
    let rows = conn.query(
        r#"
            SELECT * FROM company
            WHERE CAST(short_cik AS INT) % 1000 < 3;
        "#, &[]
    ).expect("Couldn't get companies");
    rows.iter().map(|row| Company {
        short_cik: row.get("short_cik"),
        company_name: row.get("company_name"),
        created_at: Some(row.get("created_at")),
        updated_at: Some(row.get("updated_at")),
    }).collect()
}

pub fn get_company_filings(conn: &Connection, company: &Company) -> Vec<Filing> {
    let rows = conn.query(
        r#"
            SELECT * FROM filing
            WHERE company_short_cik = $1
        "#, &[&company.short_cik]
    ).expect("Couldn't get company filings");
    rows.iter().map(|row| Filing {
        id: row.get("id"),
        company_short_cik: row.get("company_short_cik"),
        filing_name: row.get("filing_name"),
        filing_edgar_url: row.get("filing_edgar_url"),
        filing_quarter: row.get("filing_quarter"),
        filing_year: row.get("filing_year"),
        collected: row.get("collected"),
        created_at: Some(row.get("created_at")),
        updated_at: Some(row.get("updated_at")),
    }).collect()
}

pub fn upsert_co_and_filings (conn: &Connection, c: &Company, filings: &Vec<Filing>) {
    let trans = conn.transaction().expect("Couldn't begin transaction");

    trans.execute(
        "
             INSERT INTO company
             (short_cik, company_name, created_at, updated_at)
             VALUES ($1, $2, $3, $4)
             ON CONFLICT (short_cik) DO NOTHING;
        ",
        &[&c.short_cik, &c.company_name, &c.created_at, &c.updated_at]
    ).expect("Couldn't insert company");

    let filing_upsert_stmt = trans
        .prepare(
            "
             INSERT INTO filing
             (id, company_short_cik, filing_name, filing_edgar_url, filing_quarter, filing_year, created_at, updated_at, collected)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9);
             ",
        )
        .expect("Couldn't prepare filing upsert statement for execution");
    for f in filings.iter() {
        filing_upsert_stmt.execute(
            &[&f.id, &f.company_short_cik, &f.filing_name, &f.filing_edgar_url,
                &f.filing_quarter, &f.filing_year, &f.created_at, &f.updated_at, &f.collected
            ]
        ).expect("Couldn't perform update");
    }

    trans.commit().expect("Couldn't commit transaction");
}
