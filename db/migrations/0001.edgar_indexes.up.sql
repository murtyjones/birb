CREATE OR REPLACE FUNCTION trigger_set_timestamp()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TABLE IF NOT EXISTS edgar_indexes (
  id SERIAL NOT NULL PRIMARY KEY
  ,index_name VARCHAR (30) NOT NULL
  ,index_year INTEGER
  ,index_quarter INTEGER
  ,created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
  ,updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER set_timestamp
BEFORE UPDATE ON edgar_indexes
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();
