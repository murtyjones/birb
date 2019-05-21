# Filing Metadata
The SEC makes metadata about company filings available for programmatic download [here](http://www.sec.gov/Archives/edgar/full-index/).

This index is updated daily with new filings for each quarter until the final day of the quarter.

## Index Structure
`master.idx`, which this crate uses, looks like this:
```
Description:           Master Index of EDGAR Dissemination Feed
Last Data Received:    March 31, 2016
Comments:              webmaster@sec.gov
Anonymous FTP:         ftp://ftp.sec.gov/edgar/
Cloud HTTP:            https://www.sec.gov/Archives/

 
 
 
CIK|Company Name|Form Type|Date Filed|Filename
--------------------------------------------------------------------------------
1000032|BINCH JAMES G|4|2016-03-02|edgar/data/1000032/0001209191-16-104477.txt
1000032|BINCH JAMES G|4|2016-03-11|edgar/data/1000032/0001209191-16-107917.txt
1000045|NICHOLAS FINANCIAL INC|10-Q|2016-02-09|edgar/data/1000045/0001193125-16-454777.txt
1000045|NICHOLAS FINANCIAL INC|8-K/A|2016-02-01|edgar/data/1000045/0001193125-16-445158.txt
1000045|NICHOLAS FINANCIAL INC|8-K|2016-01-28|edgar/data/1000045/0001193125-16-440817.txt
1000045|NICHOLAS FINANCIAL INC|SC 13G/A|2016-02-16|edgar/data/1000045/0001193125-16-465272.txt

(~380,000 more rows)
```

### Parsing the Index
#### Pseudocode
For everything after line 11 (`-----`...), we parse using the `|` delimiter and deserialize into a struct.
### Storing the Filing Metadata
1. Check if the index has already been processed by searching for it by `year` and `quarter` in the **`edgar_index`** table. If it has been processed (IE `status` = `PROCESSED`), skip the next steps. Otherwise, parse it (see `Parsing the Index` section) and do the following for each row in the index:
    1. **Upsert the `edgar_index` table:**
        - `index_name` (e.g. `master.idx`)
        - `index_year` (e.g. `2018`) <-- Integer
        - `index_quarter` (e.g. `3`) <-- Integer
        - `status` (e.g. null) <-- enum (`PROCESSED`, `FAILED`, `null`)
    1. **Upsert the company into the `companies` table, with the following fields:**
        - `name` (e.g. `1 800 FLOWERS COM INC`)
        - `cik` (e.g. `0001084869`) <-- Primary Key for the table
        - `short_cik` (e.g. `1084869`)
    2. **Upsert the filing into the `filing_metadata` tables with the following values:**
        - `cik` (e.g. `0001084869`) <-- Foreign Key (to `companies` table)
        - `date_filed` (e.g. `2018-09-14`) <-- Date field type
        - `form_type` (e.g. `10-K`) <-- String
        - `file_short_url` (e.g. `edgar/data/1084869/0001437749-18-017027.txt`)
2. Once all of these upsertions are done for each company, update the **`edgar_index`** table:
    - `status` (`PROCESSED`)
