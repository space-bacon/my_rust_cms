CREATE TABLE settings (
    id SERIAL PRIMARY KEY,
    setting_key VARCHAR NOT NULL UNIQUE,
    setting_value TEXT,
    created_at TIMESTAMP DEFAULT NOW()
);
