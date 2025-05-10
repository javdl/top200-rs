-- Migration: V4__create_market_caps_table.sql

CREATE TABLE market_caps (
    id SERIAL PRIMARY KEY,
    ticker TEXT NOT NULL,
    name TEXT,
    market_cap_original BIGINT,
    original_currency TEXT, -- Kept as TEXT to align with current usage, CHAR(3) might be better
    market_cap_eur BIGINT,
    market_cap_usd BIGINT,
    exchange TEXT,
    active BOOLEAN,
    api_timestamp BIGINT NOT NULL, -- The timestamp of this data snapshot
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (ticker, api_timestamp)
);

-- Trigger for updated_at
-- Assuming trigger_set_timestamp() function was created in V1
CREATE TRIGGER set_timestamp_market_caps
BEFORE UPDATE ON market_caps
FOR EACH ROW
EXECUTE FUNCTION trigger_set_timestamp();