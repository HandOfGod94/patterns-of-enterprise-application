-- Your SQL goes here
CREATE TABLE IF NOT EXISTS contracts
(
    id         BIGSERIAL PRIMARY KEY,
    product    BIGINT,
    revenue    DECIMAL,
    dateSigned DATE
);