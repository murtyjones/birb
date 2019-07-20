provider "aws" {
  version                 = ">=2.5.0"
  shared_credentials_file = "$HOME/.aws/credentials"
  profile                 = "birb"
  region                  = "${var.aws_region}"
}
