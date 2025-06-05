-- Add migration script here
CREATE TABLE IF NOT EXISTS ticker_details (
    ticker TEXT PRIMARY KEY,
    description TEXT,
    homepage_url TEXT,
    employees TEXT,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Note: We keep the columns in market_caps for backward compatibility
-- The description and homepage_url columns in market_caps will be populated by ticker_details table when needed
