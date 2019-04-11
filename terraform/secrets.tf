resource "aws_secretsmanager_secret" "ROCKET_ENV" {
  name = "ROCKET_ENV"
  recovery_window_in_days = 0
}

resource "aws_secretsmanager_secret" "ROCKET_PORT" {
  name = "ROCKET_PORT"
  recovery_window_in_days = 0
}

resource "aws_secretsmanager_secret" "ROCKET_DATABASES" {
  name = "ROCKET_DATABASES"
  recovery_window_in_days = 0
}
