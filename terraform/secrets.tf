resource "aws_secretsmanager_secret" "ROCKET_ENV" {
  name = "ROCKET_ENV"
}

resource "aws_secretsmanager_secret" "ROCKET_PORT" {
  name = "ROCKET_PORT"
}

resource "aws_secretsmanager_secret" "ROCKET_DATABASES" {
  name = "ROCKET_DATABASES"
}
