-- Migration: V5__add_price_to_market_caps.sql

ALTER TABLE market_caps
ADD COLUMN price DOUBLE PRECISION;