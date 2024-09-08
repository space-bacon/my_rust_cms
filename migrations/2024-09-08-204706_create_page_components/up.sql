CREATE TABLE page_components (
    id SERIAL PRIMARY KEY,
    page_id INTEGER REFERENCES pages(id),
    component_id INTEGER REFERENCES components(id),
    position INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);
