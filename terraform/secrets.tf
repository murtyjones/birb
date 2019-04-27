resource "aws_secretsmanager_secret" "ROCKET_DATABASES" {
  name                    = "ROCKET_DATABASES"
  recovery_window_in_days = 0
}

resource "aws_secretsmanager_secret_version" "ROCKET_DATABASES" {
  secret_id     = "${aws_secretsmanager_secret.ROCKET_DATABASES.id}"
  secret_string = "${var.ROCKET_DATABASES}"
}
