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

build-binary:
    # get base image
	docker pull clux/muslrust
	# build binary
	docker run --rm -v cargo-cache:/root/.cargo \
		-v $$PWD:/volume \
		-w /volume \
		-it clux/muslrust \
		cargo build -p api --release

copy-artifacts:
	# put binary and production dockerfile in a temporary
	# folder to keep the build context simple/small
	rm -rf out
	mkdir out
	cp ./crates/api/Dockerfile-prod out
	cp ./target/x86_64-unknown-linux-musl/release/api out


build-push-docker-image:
	./scripts/build_and_push.sh

release-tag-latest:
	docker tag $(REPO)/$(NAME):$(VERSION) $(REPO)/$(NAME):latest

ecr-login:
	eval $(aws ecr get-login --no-include-email --region $AWS_DEFAULT_REGION | sed 's|https://||')

ecr-push:
	docker tag birb/api 757879768810.dkr.ecr.us-east-1.amazonaws.com/birb-api
	docker push 757879768810.dkr.ecr.us-east-1.amazonaws.com/birb-api
