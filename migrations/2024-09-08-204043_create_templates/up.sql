CREATE TABLE templates (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL UNIQUE,
    layout TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP
);
