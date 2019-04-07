default: dev

dev:
	make down
	make build
	make up
	cargo watch -x "run --package api"

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
