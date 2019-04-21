CREATE TABLE filer (
    cik VARCHAR (20) PRIMARY KEY,
    names text[]
);

INSERT INTO filer (cik, names) VALUES ('1', '{"hello"}')
