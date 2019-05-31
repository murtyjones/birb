-- Used for updated_at field auto-update
CREATE OR REPLACE FUNCTION trigger_set_timestamp()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TABLE IF NOT EXISTS ticker (
    ticker VARCHAR (10) NOT NULL
    , exchange VARCHAR (15)
    , company_short_cik VARCHAR (10) NOT NULL REFERENCES company(short_cik)
    , created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
    , updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
    , PRIMARY KEY (ticker, exchange, company_short_cik)
);

CREATE TRIGGER set_timestamp
BEFORE UPDATE ON ticker
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();
