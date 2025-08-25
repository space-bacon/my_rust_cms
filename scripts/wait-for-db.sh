#!/bin/bash

# Wait for PostgreSQL to be ready
# Usage: ./wait-for-db.sh [host] [port] [user] [database]

HOST=${1:-localhost}
PORT=${2:-5432}
USER=${3:-rustcms}
DATABASE=${4:-my_rust_cms}

echo "⏳ Waiting for PostgreSQL at $HOST:$PORT..."

# Function to test database connection
test_db() {
    PGPASSWORD=$POSTGRES_PASSWORD psql -h "$HOST" -p "$PORT" -U "$USER" -d "$DATABASE" -c '\q' 2>/dev/null
}

# Wait up to 60 seconds for database to be ready
TIMEOUT=60
COUNTER=0

until test_db; do
    COUNTER=$((COUNTER + 1))
    if [ $COUNTER -gt $TIMEOUT ]; then
        echo "❌ Database connection timeout after ${TIMEOUT} seconds"
        exit 1
    fi
    echo "   Attempt $COUNTER/$TIMEOUT - Database not ready yet..."
    sleep 1
done

echo "✅ PostgreSQL is ready at $HOST:$PORT"
