-- Used for updated_at field auto-update
CREATE OR REPLACE FUNCTION trigger_set_timestamp()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TABLE IF NOT EXISTS filing_type (
    filing_name VARCHAR (10) PRIMARY KEY
    , description TEXT
    , is_processable BOOLEAN NOT NULL DEFAULT FALSE
    , created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
    , updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER set_timestamp
BEFORE UPDATE ON filing_type
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();
