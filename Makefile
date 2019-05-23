#aws-env-vars:
#	export AWS_DEFAULT_REGION=$(aws configure get default.region)
#	export AWS_ACCOUNT_ID=$(aws configure get default.aws_account_id)

make fmeta:
	DATABASE_URI=postgres://postgres:develop@localhost:5432/postgres \
	AWS \
	cargo run -p filing-metadata
