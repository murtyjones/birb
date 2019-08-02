use postgres::{Connection, TlsMode};
use postgres::transaction::Transaction;
use models::{CompanyFilingJoined};

pub fn get_connection<S>(host: S) -> Connection where S: Into<String> {
    Connection::connect(host.into(), TlsMode::None).unwrap()
}

pub fn truncate_local_companies(conn: &Connection) {
    conn.query(
        r#"
            TRUNCATE company CASCADE;
        "#, &[]
    ).expect("Couldn't truncate `company`");
    conn.query(
        r#"
            TRUNCATE filing CASCADE;
        "#, &[]
    ).expect("Couldn't truncate `filing`");
}

pub fn get_companies_and_filings(conn: &Connection) -> Vec<CompanyFilingJoined> {
    // Get a small slice of companies
    let rows = conn.query(
        r#"
            SELECT
                c.short_cik,
                c.company_name,
                c.created_at as company_created_at,
                c.updated_at as company_updated_at,
                f.id as filing_id,
                f.filing_name,
                f.filing_quarter,
                f.filing_year,
                f.filing_edgar_url,
                f.collected,
                f.created_at as filing_created_at,
                f.updated_at as filing_updated_at
            FROM company AS c
            JOIN filing AS f ON c.short_cik = f.company_short_cik
            WHERE CAST(short_cik AS INT) % 100 <= 5;
        "#, &[]
    ).expect("Couldn't get companies");
    rows.iter().map(|row| CompanyFilingJoined {
        short_cik: row.get("short_cik"),
        company_name: row.get("company_name"),
        company_created_at: Some(row.get("company_created_at")),
        company_updated_at: Some(row.get("company_updated_at")),
        id: row.get("filing_id"),
        filing_name: row.get("filing_name"),
        filing_edgar_url: row.get("filing_edgar_url"),
        filing_quarter: row.get("filing_quarter"),
        filing_year: row.get("filing_year"),
        collected: row.get("collected"),
        filing_created_at: Some(row.get("filing_created_at")),
        filing_updated_at: Some(row.get("filing_updated_at")),
    }).collect()
}

pub fn upsert_company_and_filings (trans: &Transaction, company_with_filing: &CompanyFilingJoined) {
    trans.execute(
        "
             INSERT INTO company
             (short_cik, company_name, created_at, updated_at)
             VALUES ($1, $2, $3, $4)
             ON CONFLICT (short_cik) DO NOTHING;
        ",
        &[&company_with_filing.short_cik, &company_with_filing.company_name,
            &company_with_filing.company_created_at, &company_with_filing.company_updated_at
        ]
    ).expect("Couldn't insert company");
    trans.execute(
        "
             INSERT INTO filing
             (id, company_short_cik, filing_name, filing_edgar_url, filing_quarter, filing_year, created_at, updated_at, collected)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9);
        ",
        &[&company_with_filing.id, &company_with_filing.short_cik,
            &company_with_filing.filing_name, &company_with_filing.filing_edgar_url,
            &company_with_filing.filing_quarter, &company_with_filing.filing_year,
            &company_with_filing.filing_created_at, &company_with_filing.filing_updated_at,
            &company_with_filing.collected
        ]
    ).expect("Couldn't insert filing");
}
