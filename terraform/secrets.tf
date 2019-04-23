resource "aws_secretsmanager_secret" "ROCKET_DATABASES" {
  name = "ROCKET_DATABASES"
  recovery_window_in_days = 0
}
