variable "aws_region" {
  description = "The AWS region things are created in"
  default = "us-east-1"
}

variable "az_count" {
  description = "Number of AZs to cover in a given region"
  default     = "2"
}

variable "app_port" {
  description = "Port exposed by the docker image to redirect traffic to"
  default     = 10050
}

variable "app_count" {
  description = "Number of docker containers to run"
  default     = 3
}

variable "health_check_path" {
  default = "/"
}

variable "fargate_cpu" {
  description = "Fargate instance CPU units to provision (1 vCPU = 1024 CPU units)"
  default     = "1024"
}

variable "fargate_memory" {
  description = "Fargate instance memory to provision (in MiB)"
  default     = "2048"
}

variable "rds_username" {
  description = "User name for RDS"
  default     = "pguser"
}

variable "rds_password" {
  description = "Password for RDS"
}

variable "rds_db_name" {
  description = "The DB name in the RDS instance. Note that this cannot contain -'s"
  default     = "datastore"
}

variable "rds_instance" {
  description = "The size of RDS instance, eg db.t2.micro"
  default     = "db.t3.micro"
}

variable "multi_az" {
  description = "Whether to deploy RDS and ECS in multi AZ mode or not"
  default     = true
}

variable "birb_api_certificate_arn" {
  description = "The certificate ARN for the birb API"
}