# Builds the production-ready API binary
build-release:
    # get base image
	docker pull clux/muslrust:nightly
	# build binary
	docker run --rm \
		-v cargo-cache:/usr/local/cargo \
		-v target-cache:$$PWD/target \
		-it clux/muslrust:nightly \
		cargo build -p $(package) --release


#		-v $$PWD:/volume \
#		-w /volume \
