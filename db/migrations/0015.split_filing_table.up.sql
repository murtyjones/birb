CREATE TABLE IF NOT EXISTS split_filing (
    filing_id SERIAL REFERENCES filing(id) NOT NULL
    , sequence INTEGER NOT NULL
    , doc_type VARCHAR (50) NOT NULL
    , filename VARCHAR (50) NOT NULL
    , description VARCHAR (50) DEFAULT NULL
    , s3_url_prefix VARCHAR (70) NOT NULL
    , created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
    , updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
    , UNIQUE (filing_id, filename)
    , PRIMARY KEY (filing_id, sequence)
);

CREATE TRIGGER set_timestamp
BEFORE UPDATE ON split_filing
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();
