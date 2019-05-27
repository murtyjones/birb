use models::Company;
use postgres::Connection;

/// Find an entity using its cik
pub fn get_autocomplete_results(conn: &Connection, substr: String) -> Result<Vec<Company>, &str> {
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
        });
    }
    Ok(binds)
}

#[cfg(test)]
mod test {
    #[test]
    fn get_autocomplete_results_unwraps() {}
}
