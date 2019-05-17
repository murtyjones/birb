# Filing Metadata
The SEC makes metadata about company filings available for programmatic download [here](http://www.sec.gov/Archives/edgar/full-index/).

This index is updated daily with new filings for each quarter until the final day of the quarter.

## Index Structure
`company.idx`, which this crate uses, looks like this:
```
Description:           Master Index of EDGAR Dissemination Feed by Company Name
Last Data Received:    September 30, 2018
Comments:              webmaster@sec.gov
Anonymous FTP:         ftp://ftp.sec.gov/edgar/
 
 
 
 
Company Name                                                  Form Type   CIK         Date Filed  File Name
---------------------------------------------------------------------------------------------------------------------------------------------
'Laine's Bake Shop LLC, Series of BG Consortium LLC           C/A         1732207     2018-08-16  edgar/data/1732207/0001670254-18-000376.txt         
01VC Fund II, L.P.                                            D           1746009     2018-07-20  edgar/data/1746009/0001213900-18-009448.txt         
1 800 FLOWERS COM INC                                         10-K        1084869     2018-09-14  edgar/data/1084869/0001437749-18-017027.txt

(~195,000 more rows)
```

## Parsing the Index
1. Check if the index has already been processed by searching for it by `year` and `quarter` in the **`edgar_indexes`** table. If it has been processed (IE `status` = `PROCESSED`), skip the next steps. Otherwise do the following for each row in the index:
    1. **Upsert the `edgar_indexes` table:**
        - `index_name` (e.g. `company.idx`)
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
2. Once all of these upsertions are done for each company, update the **`edgar_indexes`** table:
    - `status` (`PROCESSED`)
