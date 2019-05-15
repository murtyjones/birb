# Builds the production-ready API binary
build-release:
    # get base image
	docker pull clux/muslrust:nightly
	# build binary
	docker run --rm \
		-v cargo-cache:/usr/local/cargo \
		-v cargo-bin-cache:$$HOME/.cargo/bin \
		-v $$PWD:/volume \
		-w /volume \
		-it clux/muslrust:nightly \
		cargo build -p $(package) --release

#aws-env-vars:
#	export AWS_DEFAULT_REGION=$(aws configure get default.region)
#	export AWS_ACCOUNT_ID=$(aws configure get default.aws_account_id)
