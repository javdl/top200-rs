-- Migration: V1__create_initial_tables.sql

-- Table for storing company details
CREATE TABLE company_details (
    ticker TEXT PRIMARY KEY,
    name TEXT,
    market_cap DOUBLE PRECISION,
    currency_name TEXT,
    currency_symbol TEXT,
    active BOOLEAN,
    description TEXT,
    homepage_url TEXT,
    weighted_shares_outstanding DOUBLE PRECISION,
    employees TEXT,
    revenue DOUBLE PRECISION,
    revenue_usd DOUBLE PRECISION,
    api_timestamp TIMESTAMPTZ, -- Renamed from 'timestamp' in model to avoid confusion
    working_capital_ratio DOUBLE PRECISION,
    quick_ratio DOUBLE PRECISION,
    eps DOUBLE PRECISION,
    pe_ratio DOUBLE PRECISION,
    debt_equity_ratio DOUBLE PRECISION,
    roe DOUBLE PRECISION,
    extra_data JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Trigger to automatically update 'updated_at' timestamp on row update
CREATE OR REPLACE FUNCTION trigger_set_timestamp()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER set_timestamp_company_details
BEFORE UPDATE ON company_details
FOR EACH ROW
EXECUTE FUNCTION trigger_set_timestamp();

-- Table for storing exchange rates
CREATE TABLE exchange_rates (
    id SERIAL PRIMARY KEY,
    from_currency CHAR(3) NOT NULL,
    to_currency CHAR(3) NOT NULL,
    rate DOUBLE PRECISION NOT NULL,
    timestamp DATE NOT NULL, -- Or TIMESTAMPTZ if time is important for the rate
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (from_currency, to_currency, timestamp) -- Ensure unique rate per day per pair
);

-- Table for storing historical market capitalization data
CREATE TABLE historical_market_caps (
    id SERIAL PRIMARY KEY,
    ticker TEXT NOT NULL, -- Could be a foreign key to company_details.ticker
    date DATE NOT NULL,
    market_cap DOUBLE PRECISION,
    currency CHAR(3), -- Store the currency of this market cap entry
    year INTEGER, -- Extracted from date for easier querying
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (ticker, date)
    -- CONSTRAINT fk_company_details FOREIGN KEY(ticker) REFERENCES company_details(ticker) -- Consider adding if ticker table always populated first
);

CREATE TRIGGER set_timestamp_historical_market_caps
BEFORE UPDATE ON historical_market_caps
FOR EACH ROW
EXECUTE FUNCTION trigger_set_timestamp();