use aws::s3;
use failure;
use models::{Company, CompanyFilingData, Filing};
use postgres::Connection;

/// Model for an S3 link for a filing
#[derive(Debug, Serialize, Deserialize)]
pub struct FilingS3Link {
    signed_url: Option<String>,
}

fn get_signed_url<S: Into<String>>(filing_edgar_url: S) -> String {
    s3::get_signed_url("birb-edgar-filings", &*filing_edgar_url.into())
}

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
pub fn get_filing_s3_link(
    conn: &Connection,
    filing_id: i32,
) -> Result<FilingS3Link, failure::Error> {
    let rows = &conn
        .query(
            "
            SELECT * FROM filing
            WHERE id = $1;
        ",
            &[&filing_id],
        )
        .expect("Couldn't get company's filing info for signed url");
    if rows.len() == 0 {
        return Ok(FilingS3Link { signed_url: None });
    }

    let filing_link = match rows.len() {
        0 => FilingS3Link { signed_url: None },
        _ => {
            let filing_edgar_url: String = rows.get(0).get("filing_edgar_url");
            FilingS3Link {
                signed_url: Some(get_signed_url(filing_edgar_url)),
            }
        }
    };

    Ok(filing_link)
}

#[cfg(test)]
mod test {}
