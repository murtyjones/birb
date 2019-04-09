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

release-build:
	docker pull clux/muslrust
	docker run --rm -v cargo-cache:/root/.cargo \
		-v $$PWD:/volume \
		-w /volume \
		-it clux/muslrust \
		cargo build -p api --release
	rm -rf out
	mkdir out
	cp ./crates/api/Dockerfile-prod out/Dockerfile
	cp ./target/x86_64-unknown-linux-musl/release/api out
	# Keep these commands together:
	cd out && docker build -t $(REPO)/$(NAME):$(VERSION) .

release-tag-latest:
	docker tag $(REPO)/$(NAME):$(VERSION) $(REPO)/$(NAME):latest

ecr-login:
	eval $(aws ecr get-login --no-include-email --region $AWS_DEFAULT_REGION | sed 's|https://||')

ecr-push:
	docker tag birb/api 757879768810.dkr.ecr.us-east-1.amazonaws.com/birb-api
	docker push 757879768810.dkr.ecr.us-east-1.amazonaws.com/birb-api
