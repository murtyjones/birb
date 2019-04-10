variable "aws_region" {
  description = "The AWS region things are created in"
  default = "us-east-1"
}

variable "api_image" {
  description = "Docker image to run in the ECS cluster"
  default     = "birb-api"
}
