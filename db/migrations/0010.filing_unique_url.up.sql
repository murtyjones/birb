ALTER TABLE filing
ADD CONSTRAINT unique_url UNIQUE (filing_edgar_url);



