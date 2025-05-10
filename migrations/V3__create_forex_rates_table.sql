-- Migration: V3__create_forex_rates_table.sql

CREATE TABLE forex_rates (
    id SERIAL PRIMARY KEY,
    symbol TEXT NOT NULL, -- e.g., "USD/EUR"
    ask DOUBLE PRECISION NOT NULL,
    bid DOUBLE PRECISION NOT NULL,
    api_timestamp BIGINT NOT NULL, -- Assuming this is a Unix timestamp from the API
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (symbol, api_timestamp) -- Ensure unique entry per symbol at a given API timestamp
);

-- Trigger for updated_at
-- Assuming trigger_set_timestamp() function was created in V1
CREATE TRIGGER set_timestamp_forex_rates
BEFORE UPDATE ON forex_rates
FOR EACH ROW
EXECUTE FUNCTION trigger_set_timestamp();