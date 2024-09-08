CREATE TABLE builder_components (
    id SERIAL PRIMARY KEY,
    component_name VARCHAR NOT NULL,
    component_data JSONB,
    template_id INTEGER REFERENCES templates(id),
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP
);
