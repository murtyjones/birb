data "terraform_remote_state" "network" {
  backend = "s3"

  config = {
    bucket = "terraform-state-production"
    key    = "network/terraform.tfstate"
    region = "us-east-1"
  }
}

terraform {
  backend "s3" {
    bucket = "terraform-state-production"
    key    = "network/terraform.tfstate"
    region = "us-east-1"
  }
}

