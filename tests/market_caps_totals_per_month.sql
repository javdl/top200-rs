SELECT date(timestamp, 'start of month') as month, COUNT(*) as count FROM market_caps WHERE market_cap_eur IS NOT NULL GROUP BY month ORDER BY month DESC;