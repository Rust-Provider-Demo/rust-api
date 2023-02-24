-- Your SQL goes here
CREATE TABLE PROVIDERS (
    id SERIAL PRIMARY KEY,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    npi VARCHAR NOT NULL
)