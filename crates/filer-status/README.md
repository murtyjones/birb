# Filer Status
Not all filers with a `CIK` identifier actively file financial statements (10-Ks / 10-Qs). We only care about the ones that do.

This library is used to determine whether an entity files financial statements or not.
- If a filer does file financial statements, it receives an `active` filer status
- otherwise, it receives an `inactive` filer status
## Examples
### Active Filer – Tesla, Inc.
![active filer example – Tesla](../../assets/images/docs/active_filer_example_tesla.png)
### Inactive Filer – Some guy named Kenneth Sawyer
![inactive filer example – Sawyer](../../assets/images/docs/inactive_filer_example_sawyer.png)
### Guide Level Explanation
1. Iterate through our collection of inactive or no-status filers
2. For each:
    - Search `https://www.sec.gov/cgi-bin/browse-edgar?action=getcompany&CIK=${CIK}&type=10-Q&dateb=&owner=exclude&count=40`
    - If filings are found, mark `filerStatus: 'ACTIVE'` for the company in the database
    - Otherwise, mark `filerStatus: 'INACTIVE'` for the company in the database
### Implementation Level Explanation
