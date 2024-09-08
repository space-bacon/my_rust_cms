CREATE TABLE component_events (
    id SERIAL PRIMARY KEY,
    component_id INTEGER REFERENCES components(id),
    event_type VARCHAR NOT NULL,
    event_handler TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);
