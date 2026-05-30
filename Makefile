.PHONY: fmt fmt-check clippy test check ci build run deploy docker-build docker-up docker-deploy docker-logs docker-down

fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all -- --check

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

test:
	cargo test --all-targets --all-features

check:
	cargo check

ci: fmt-check check clippy test

build:
	cargo build --release

run:
	cargo run

deploy:
	cargo run -- --deploy

docker-build:
	docker compose -f docker-compose.yml build

docker-up:
	docker compose -f docker-compose.yml up -d --build

docker-deploy:
	docker compose -f docker-compose.yml run --rm rathenafr-discord-bot --deploy

docker-logs:
	docker compose -f docker-compose.yml logs -f rathenafr-discord-bot

docker-down:
	docker compose -f docker-compose.yml down
