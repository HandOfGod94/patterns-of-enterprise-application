-- Your SQL goes here
CREATE TABLE IF NOT EXISTS Product
(
    id   BIGSERIAL PRIMARY KEY,
    name VARCHAR,
    type VARCHAR
);