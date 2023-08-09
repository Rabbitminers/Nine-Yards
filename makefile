# Targets that are not associated with files
.PHONY: clean test

# Clean up backend artifacts and databases
clean:
	@echo "Cleaning backend..."
	cd backend && cargo sqlx database drop
	cd backend && cargo sqlx database create
	cd backend && cargo sqlx migrate run
	@echo "Backend cleaned and databases reset"

# Run tests for backend and web
test: backend-test web-test

# Run backend tests
backend-test:
	@echo "Running backend tests..."
	cd backend && cargo test

# Run web tests
web-test:
	@echo "Running web tests..."
	cd web && pnpm run tests

# Build backend
backend-build:
	@echo "Building backend"
	cd backend && cargo build --release
	@echo "Finished building backend"

# Build website
frontend-build:
	@echo "Building frontend"
	cd web && pnpm install
	cd web && pnpm run build
	@echo "Finished building backend"

# Build both front and backend
build: backend-build frontend-build
