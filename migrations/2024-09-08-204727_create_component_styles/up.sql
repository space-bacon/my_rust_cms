CREATE TABLE component_styles (
    id SERIAL PRIMARY KEY,
    component_id INTEGER REFERENCES components(id),
    css TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP
);
