CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL UNIQUE,
    password VARCHAR NOT NULL,
    email VARCHAR UNIQUE,
    created_at TIMESTAMP DEFAULT NOW()
);


