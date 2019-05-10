# Builds the production-ready API binary
build-release:
    # get base image
	docker pull clux/muslrust:nightly
	# build binary
	docker run --rm \
		-v ./tmp-cargo:/usr/local/cargo \
		-v ./tmp-cargo:./target \
		-v $$PWD:/volume \
		-w /volume \
		-it clux/muslrust:nightly \
		cargo build -p $(package) --release
