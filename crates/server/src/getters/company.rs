use aws::s3;
use failure;
use models::SignedUrl;
use models::{Company, CompanyFilingData, Filing, SplitDocument};
use postgres::Connection;

/// Find an entity using its cik
pub fn get_typeahead_results(conn: &Connection, substr: String) -> Result<Vec<Company>, &str> {
    let rows = &conn
        .query(
            "
        SELECT * FROM company
        WHERE company_name ILIKE ($1 || '%')
        LIMIT 10;
        ",
            &[&substr],
        )
        .expect("Couldn't search for a company");
    let mut binds = Vec::new();
    for row in rows {
        binds.push(Company {
            short_cik: row.get("short_cik"),
            company_name: row.get("company_name"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        });
    }
    Ok(binds)
}

/// Get a company's filing info
pub fn get_company_filings(
    conn: &Connection,
    short_cik: String,
) -> Result<CompanyFilingData, failure::Error> {
    let rows = &conn
        .query(
            "
            SELECT * FROM company c
            JOIN filing f ON c.short_cik = f.company_short_cik
            WHERE short_cik = $1
            AND collected = true;
        ",
            &[&short_cik],
        )
        .expect("Couldn't get company's filing info");

    let mut company_filing_info = match rows.len() {
        0 => CompanyFilingData {
            short_cik: short_cik,
            company_name: String::new(), // TODO still provide the name
            filings: Vec::new(),
        },
        _ => CompanyFilingData {
            short_cik: rows.get(0).get("short_cik"),
            company_name: rows.get(0).get("company_name"),
            filings: Vec::new(),
        },
    };

    for row in rows {
        company_filing_info.filings.push(Filing {
            id: row.get("id"),
            company_short_cik: row.get("company_short_cik"),
            collected: row.get("collected"),
            filing_edgar_url: row.get("filing_edgar_url"),
            filing_name: row.get("filing_name"),
            filing_quarter: row.get("filing_quarter"),
            filing_year: row.get("filing_year"),
            date_filed: row.get("date_filed"),
            created_at: Some(row.get("created_at")),
            updated_at: Some(row.get("updated_at")),
        });
    }
    Ok(company_filing_info)
}

/// Get a company's filing S3 link
pub fn get_signed_url(
    conn: &Connection,
    short_cik: i32,
    filing_id: i32,
) -> Result<SignedUrl, failure::Error> {
    let rows = &conn
        .query(
            "
            SELECT * FROM split_filing
            WHERE filing_id = $1
            AND sequence = 1;
        ",
            &[&filing_id],
        )
        .expect("Couldn't get company's filing info for signed url");
    assert_eq!(1, rows.len(), "No filing found!");

    let split_doc: SplitDocument = rows.get(0).into();

    // e.g. edgar/data/14707/0000014707-16-000090 + / + s102614_10q.htm
    let filepath = format!("{}/{}", split_doc.s3_url_prefix, split_doc.filename);
    let signed_url = s3::get_signed_url("birb-edgar-filings", &*filepath);

    Ok(SignedUrl {
        filing_id,
        short_cik,
        signed_url,
    })
}

#[cfg(test)]
mod test {}
