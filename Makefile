up:
	docker compose up -d --remove-orphans

stop:
	docker compose stop

down:
	docker compose down -v --remove-orphans

standalone:
	docker compose --profile standalone pull
	docker compose --profile standalone up -d --remove-orphans

lint:
	cargo clippy --all-features -- -D warnings

dev:
	COBASE_LOG=debug cargo run serve -c config/cobase.yml

migrate:
	SQLX_OFFLINE=true cargo run migrate -c config/cobase.yml

openapi:
	cargo run openapi -c config/cobase.yml

prepare:
	cargo sqlx prepare --merged
