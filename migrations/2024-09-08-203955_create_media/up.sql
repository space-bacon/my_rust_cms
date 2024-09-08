CREATE TABLE media (
    id SERIAL PRIMARY KEY,
    file_name VARCHAR NOT NULL,
    url VARCHAR NOT NULL,
    media_type VARCHAR,
    uploaded_at TIMESTAMP DEFAULT NOW(),
    user_id INTEGER REFERENCES users(id)
);
