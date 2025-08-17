-- SPDX-FileCopyrightText: 2025 Joost van der Laan <joost@fashionunited.com>
--
-- SPDX-License-Identifier: AGPL-3.0-only

-- Create symbol_changes table to track ticker symbol changes
CREATE TABLE IF NOT EXISTS symbol_changes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    old_symbol TEXT NOT NULL,
    new_symbol TEXT NOT NULL,
    change_date TEXT,
    company_name TEXT,
    reason TEXT,
    applied INTEGER DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(old_symbol, new_symbol, change_date)
);

-- Create index for faster lookups
CREATE INDEX IF NOT EXISTS idx_symbol_changes_old_symbol ON symbol_changes(old_symbol);
CREATE INDEX IF NOT EXISTS idx_symbol_changes_applied ON symbol_changes(applied);