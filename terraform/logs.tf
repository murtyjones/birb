# Set up cloudwatch group and log stream and retain logs for 30 days
resource "aws_cloudwatch_log_group" "api_log_group" {
  name              = "/ecs/birb-api"
  retention_in_days = 30

  tags {
    Name = "birb-api-log-group"
  }
}

resource "aws_cloudwatch_log_stream" "api_log_stream" {
  name           = "birb-api-log-stream"
  log_group_name = "${aws_cloudwatch_log_group.api_log_group.name}"
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
