CREATE TABLE components (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    template_id INTEGER REFERENCES templates(id),
    component_data JSONB NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP
);
