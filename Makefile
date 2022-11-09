up:
	docker compose up -d

lint:
	cargo clippy --all-features -- -D warnings
