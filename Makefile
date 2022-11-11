up:
	docker compose up -d --remove-orphans

lint:
	cargo clippy --all-features -- -D warnings

serve:
	cargo run serve -c config/cobase.yml

migrate:
	cargo run migrate -c config/cobase.yml

openapi:
	cargo run openapi -c config/cobase.yml
