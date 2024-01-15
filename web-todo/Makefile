.PHONY: test
test:
	@echo "Running tests..."
	@cargo test

.PHONY: run
run:
	@echo "Running..."
	@cargo run

.PHONY: dev
dev:
	@echo "Running in dev mode..."
	sqlx db create
	sqlx migrate run
	@cargo watch -x run

.PHONY: psql
psql:
	@echo "Connecting to database..."
	docker compose exec database psql -U postgres -d todos
