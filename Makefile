NAME=api
VERSION=$(shell git rev-parse HEAD)
SEMVER_VERSION=$(shell grep version Cargo.toml | awk -F"\"" '{print $$2}' | head -n 1)
REPO=birb

default: dev

dev:
	make down
	make build
	make up
	cargo watch -x "run --package api"

rebuild:
	make down
	make build
	make up

test:
	make down
	make build
	docker-compose up -d
	docker-compose run --rm test cargo test --all

down:
	docker-compose down

up:
	@echo "=============starting server locally============="
	docker-compose up -d --scale test=0

build:
	docker-compose build

logs:
	docker-compose logs -f

clean: down
	@echo "=============cleaning up============="
	docker system prune -f
	docker volume prune -f

build-and-push:
	docker pull clux/muslrust
	docker run --rm -v cargo-cache:/root/.cargo \
		-v $$PWD:/volume \
		-w /volume \
		-it clux/muslrust \
		cargo build -p api --release
	docker build -f crates/api/Dockerfile-prod -t $(REPO)/$(NAME):$(VERSION) .
