variable "aws_region" {
  description = "The AWS region things are created in"
  default = "us-east-1"
}

variable "app_image" {
  description = "Docker image to run in the ECS cluster"
  default     = "murtyjones/birb:latest"
}
