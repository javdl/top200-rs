CREATE TABLE IF NOT EXISTS currencies (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    symbol TEXT,
    exchange_rate_usd REAL,
    last_updated TIMESTAMP
);

-- Insert some common currencies
INSERT OR IGNORE INTO currencies (code, name, symbol, exchange_rate_usd, last_updated)
VALUES 
    ('USD', 'US Dollar', '$', 1.0, CURRENT_TIMESTAMP),
    ('EUR', 'Euro', '€', NULL, NULL),
    ('GBP', 'British Pound', '£', NULL, NULL),
    ('JPY', 'Japanese Yen', '¥', NULL, NULL); 