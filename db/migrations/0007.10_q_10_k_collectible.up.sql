UPDATE filing_type
SET collectible = true
WHERE filing_name = '10-Q'
OR filing_name = '10-K';
