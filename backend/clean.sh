# Remove existing database
cargo sqlx database drop
echo "Removed existing database"

# Create new instance and run init migrations
cargo sqlx database create
echo "Created new database"

cargo sqlx migrate run
echo "Ran setup migrations"