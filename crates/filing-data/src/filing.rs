use filing_metadata_lib::time_periods::Quarter;
use filing_metadata_lib::time_periods::Year;

#[derive(Debug, FromSql)]
pub struct Filing {
    pub id: u32,
    pub company_short_cik: String,
    pub filing_name: String,
    pub filing_edgar_url: String,
    pub filing_quarter: Quarter,
    pub filing_year: Year,
}
