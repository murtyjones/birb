use filing_metadata_lib::time_periods::Quarter;
use filing_metadata_lib::time_periods::Year;

#[derive(Debug, FromSql)]
pub struct Filing {
    pub id: i32,
    pub company_short_cik: String,
    pub filing_name: String,
    pub filing_edgar_url: String,
    pub filing_quarter: i32,
    pub filing_year: i32,
}
