-- Migration: V2__create_currencies_table.sql

CREATE TABLE currencies (
    code CHAR(3) PRIMARY KEY, -- Standard currency codes are 3 chars
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Trigger to automatically update 'updated_at' timestamp
-- Assuming trigger_set_timestamp() function was created in V1
CREATE TRIGGER set_timestamp_currencies
BEFORE UPDATE ON currencies
FOR EACH ROW
EXECUTE FUNCTION trigger_set_timestamp();