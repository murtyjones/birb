// For lamdba:
resource "aws_ssm_parameter" "ROCKET_DATABASES" {
  name        = "/${var.environment}/database/ROCKET_DATABASES"
  description = "Database connection info for Rocket / ECS"
  type        = "SecureString"
  value       = "${var.ROCKET_DATABASES}"

  tags = {
    environment = "${var.environment}"
  }
}

// For everything else:
resource "aws_secretsmanager_secret" "ROCKET_DATABASES" {
  name                    = "ROCKET_DATABASES"
  recovery_window_in_days = 0
}


resource "aws_secretsmanager_secret_version" "ROCKET_DATABASES" {
  secret_id     = "${aws_secretsmanager_secret.ROCKET_DATABASES.id}"
  secret_string = "${var.ROCKET_DATABASES}"
}
