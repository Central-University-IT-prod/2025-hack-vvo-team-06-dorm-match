#!/bin/bash
set -e

# Run migrations
echo "Running database migrations..."
/root/.cargo/bin/sqlx migrate run --database-url "$DATABASE_URL" --source /app/migrations

# Start the application
exec "$@"
