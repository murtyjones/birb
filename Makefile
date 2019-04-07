default: dev

dev:
	make rebuild
	cargo watch -x "run --package api"

rebuild:
	make down
	make build
	make up

test:
	make rebuild
	cargo test --all

down:
	docker-compose down

up:
	@echo "=============starting server locally============="
	docker-compose up -d

build:
	docker-compose build

logs:
	docker-compose logs -f

clean: down
	@echo "=============cleaning up============="
	docker system prune -f
	docker volume prune -f
