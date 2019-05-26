# Set up cloudwatch group and log stream and retain logs for 30 days
resource "aws_cloudwatch_log_group" "server_log_group" {
  name              = "/ecs/birb-server"
  retention_in_days = 30

  tags {
    Name = "birb-server-log-group"
  }
}

resource "aws_cloudwatch_log_stream" "server_log_stream" {
  name           = "birb-server-log-stream"
  log_group_name = "${aws_cloudwatch_log_group.server_log_group.name}"
}

# Set up cloudwatch group and log stream and retain logs for 30 days
resource "aws_cloudwatch_log_group" "edgar_log_group" {
  name              = "/ecs/birb-edgar"
  retention_in_days = 30

  tags {
    Name = "birb-edgar-log-group"
  }
}

resource "aws_cloudwatch_log_stream" "edgar_log_stream" {
  name           = "birb-edgar-log-group"
  log_group_name = "${aws_cloudwatch_log_group.edgar_log_group.name}"
}
