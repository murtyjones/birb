# Run the dev environment for local development
default: dev

# Run the dev environment for local development
dev: down up-no-tests sleep5 cargo-watch

# Run the server, watching for changes
cargo-watch:
	cargo watch -x "run -p api"

# Run tests in testing container and then shut down
test: down
	docker-compose up -d
	docker-compose run --rm test bash -c "cargo test --all"

# Start the container without tests
up-no-tests:
	docker-compose up -d --scale test=0

# Tear down docker containers
down:
	docker-compose down

# Run docker containers
up:
	docker-compose up -d

# Build the container from scratch
rebuild: 
	make down
	docker-compose build
	make up

# Show the container logs
logs:
	docker-compose logs -f

# Prune unused containers / images
clean: down
	docker system prune -f
	docker volume prune -f

# Builds the production-ready API binary
build-api-release:
    # get base image
	docker pull clux/muslrust:nightly
	# build binary
	docker run --rm -v cargo-cache:/root/.cargo \
		-v $$PWD:/volume \
		-w /volume \
		-it clux/muslrust:nightly \
		cargo build -p api --release

# put binary and production dockerfile in a temporary
# folder to keep ≤the build context simple/small
copy-artifacts:
	rm -rf out
	mkdir out
	cp ./crates/api/Dockerfile-prod out
	cp ./target/x86_64-unknown-linux-musl/release/api_bin out

# Release to production
make release: build-api-release copy-artifacts 
	./scripts/build_and_push.sh

# Deploy birb infrastructure
birb-up:
	terraform apply "plan"
	rm -rf plan

# Destroy birb infrastructure
birb-destroy:
	terraform destroy -auto-approve -var-file=terraform/secret.tfvars terraform/

# Prepare bird infrastructure for deploy
birb-plan:
	terraform plan -out=plan -var-file=terraform/secret.tfvars terraform/

# Prepare api.birb.io certificate for deployment
birb-cert-plan:
	terraform plan -out=plan tf-certificate/

# Regrettable hack used to await a healthy postgres status before attempting to
# establish a connection in Rocket. Tried waiting for 5432 to become reachable
# but that actually happens in advance of postgres becoming healthy/usable,
# so simply waiting for the port isn't an option. In the future this will ideally
# wait for a passing healthcheck of some kind a la https://github.com/peter-evans/docker-compose-healthcheck
sleep5:
	sleep 5s
