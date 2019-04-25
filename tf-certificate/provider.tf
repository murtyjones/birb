provider "aws" {
  version                 = ">= 1.33.0"
  shared_credentials_file = "$HOME/.aws/credentials"
  profile                 = "default"
  region                  = "${var.aws_region}"
}
