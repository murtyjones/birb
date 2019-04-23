default: dev

dev: down up-no-tests sleep5 cargo-watch

# Run the server, watching for changes
cargo-watch:
	cargo watch -x "run -p api"

# Run tests in testing container and then shut down
test: down up-with-tests
	docker-compose run --rm test bash -c "cargo test --all"

up-no-tests:
	docker-compose up -d --scale test=0

up-with-tests:
	docker-compose up -d

# Tear down docker containers
down:
	docker-compose down

# Run docker containers
up:
	docker-compose up -d

# Build each container
build:
	docker-compose build

rebuild: down build up

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
	# folder to keep ≤the build context simple/small
	rm -rf out
	mkdir out
	cp ./crates/api/Dockerfile-prod out
	cp ./target/x86_64-unknown-linux-musl/release/api_bin out

make build-all: build-binary copy-artifacts build-push-docker-image

make run-release: down up
	docker run -e \
		ROCKET_DATABASES='{mongo_datastore={url="mongodb://localhost:27100/playground"}}' \
		murtyjones/birb_api:latest

build-push-docker-image:
	./scripts/build_and_push.sh

tfup:
	terraform apply "plan"
	rm -rf plan

tfdown:
	terraform destroy -auto-approve terraform/

tfplan:
	terraform plan -out=plan -var-file=terraform/secret.tfvars terraform/

tfplan-cert:
	terraform plan -out=plan tf-certificate/


pg:
	docker exec -it birb_db_1 psql -U postgres

# Regrettable hack used to await a healthy postgres status before attempting to
# establish a connection in Rocket. Tried waiting for 5432 to become reachable
# but that actually happens in advance of postgres becoming healthy/usable,
# so simply waiting for the port isn't an option. In the future this will ideally
# wait for a passing healthcheck of some kind a la https://github.com/peter-evans/docker-compose-healthcheck
sleep5:
	sleep 5s
