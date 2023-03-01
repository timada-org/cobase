up:
	docker compose up -d --remove-orphans

stop:
	docker compose stop

down:
	docker compose down -v --remove-orphans

standalone:
	docker compose --profile standalone pull
	docker compose --profile standalone up -d --remove-orphans

dev:
	COBASE_LOG=debug cargo run serve -c configs/default.yml

lint:
	cargo clippy --all-features -- -D warnings

migrate:
	sqlx migrate run --source ./cmd/migrations

revert:
	sqlx migrate revert --source ./cmd/migrations

prepare:
	cargo sqlx prepare --merged

openapi:
	cargo run openapi -c configs/default.yml

fmt:
	cargo fmt -- --emit files

clippy:
	cargo clippy --all-features -- -D warnings

clippy.fix:
	cargo clippy --fix --all-features -- -D warnings
