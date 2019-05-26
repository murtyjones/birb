variable "aws_region" {
  description = "The AWS region things are created in"
  default     = "us-east-1"
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
  default     = 1
}

variable "health_check_path" {
  default = "/"
}

variable "birb_server_cpu" {
  description = "Fargate instance CPU units to provision (1 vCPU = 1024 CPU units)"
  default     = "256"
}

variable "birb_server_memory" {
  description = "Fargate instance memory to provision (in MiB)"
  default     = "512"
}

variable "birb_edgar_worker_cpu" {
  description = "Instance CPU units to provision (1 vCPU = 1024 CPU units) for the Edgar worker"
  default     = "256"
}

variable "birb_edgar_worker_memory" {
  description = "Instance memory to provision (in MiB) for the Edgar worker"
  default     = "512"
}

variable "rds_username" {
  description = "User name for RDS"
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

  // May want to flip this to true at some point:
  default = false
}

variable "birb_server_certificate_arn" {
  description = "The certificate ARN for the birb server"
}

variable "marty_ip_address_1" {
  description = "Marty's 1st IP address"
}

variable "marty_ip_address_2" {
  description = "Marty's 2nd IP address"
}

variable "marty_ip_address_3" {
  description = "Marty's 3rd IP address"
}

variable "marty_id_rsa_pub" {
  description = "Marty's public key"
}

variable "ROCKET_DATABASES" {
  description = "Database(s) connection information for Rocket"
}

variable "DATABASE_URI" {
  description = "Actual Databases connection string"
}

variable "environment" {
  description = "The environment being deployed"
  default     = "production"
}
