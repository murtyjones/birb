# Builds the production-ready API binary
build-release:
    # get base image
	docker pull clux/muslrust:nightly
	# build binary
	docker run --rm \
		-v cargo-cache:/usr/local/cargo \
		-v target-cache:$$PWD/target \
		-v $$PWD:/volume \
		-w /volume \
		-it clux/muslrust:nightly \
		RUST_BACKTRACE=1 cargo build -p $(package) --release
