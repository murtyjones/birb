-- Used for updated_at field auto-update
CREATE OR REPLACE FUNCTION trigger_set_timestamp()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TABLE IF NOT EXISTS filing (
    id SERIAL PRIMARY KEY
    , company_short_cik VARCHAR (10) REFERENCES company(short_cik)
    , filing_name VARCHAR (20) REFERENCES filing_type
    , filing_edgar_url TEXT
    , filing_quarter INTEGER
    , filing_year INTEGER
    , created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
    , updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER set_timestamp
BEFORE UPDATE ON filing
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();
