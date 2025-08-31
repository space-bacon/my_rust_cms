-- Database initialization script for Rust CMS
-- This ensures the database exists and is properly configured

-- Create the database if it doesn't exist
SELECT 'CREATE DATABASE my_rust_cms'
WHERE NOT EXISTS (SELECT FROM pg_database WHERE datname = 'my_rust_cms')\gexec

-- Connect to the database
\c my_rust_cms;

-- Create extensions if needed
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pg_stat_statements";

-- Set timezone
SET timezone = 'UTC';

-- Grant permissions to the myrustcms user
GRANT ALL PRIVILEGES ON DATABASE my_rust_cms TO myrustcms;
GRANT ALL ON SCHEMA public TO myrustcms;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO myrustcms;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public TO myrustcms;

-- Set default privileges for future objects
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT ALL ON TABLES TO myrustcms;
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT ALL ON SEQUENCES TO myrustcms;
