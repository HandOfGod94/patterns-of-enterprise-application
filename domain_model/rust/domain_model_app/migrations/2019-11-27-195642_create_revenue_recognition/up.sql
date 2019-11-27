-- Your SQL goes here
CREATE TABLE IF NOT EXISTS revenueRecognition
(
    contract     BIGINT,
    amount       BIGINT,
    recognizedOn DATE,
    PRIMARY KEY (contract, recognizedOn)
);