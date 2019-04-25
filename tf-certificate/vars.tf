variable "domain" {
  description = "Which domain to use. Service will be deployed at api.domain"
  default     = "birb.io"
}

variable "aws_region" {
  description = "The AWS region things are created in"
  default     = "us-east-1"
}
