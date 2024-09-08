CREATE TABLE page_sections (
    id SERIAL PRIMARY KEY,
    page_id INTEGER REFERENCES pages(id),
    section_name VARCHAR NOT NULL,
    content TEXT,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP
);
