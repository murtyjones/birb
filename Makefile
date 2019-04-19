default: dev

dev: down up-without-tests
	cargo watch -x "run -p api"

rebuild: down build up-without-tests

test: down up-with-tests
	docker-compose run --rm test cargo test --all

down:
	docker-compose down

up-without-tests:
	docker-compose up -d --scale test=0

up-with-tests:
	docker-compose up -d

build:
	docker-compose build

logs:
	docker-compose logs -f

clean: down
	docker system prune -f
	docker volume prune -f

build-binary:
    # get base image
	docker pull clux/muslrust:nightly
	# build binary
	docker run --rm -v cargo-cache:/root/.cargo \
		-v $$PWD:/volume \
		-w /volume \
		-it clux/muslrust:nightly \
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

tfup:
	terraform apply -auto-approve terraform/

tfdown:
	terraform destroy -auto-approve terraform/

tfplan:
	terraform plan terraform/
