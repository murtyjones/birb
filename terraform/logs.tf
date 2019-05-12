# Set up cloudwatch group and log stream and retain logs for 30 days
resource "aws_cloudwatch_log_group" "birb_api_log_group" {
  name              = "/ecs/birb-api-app"
  retention_in_days = 30

  tags {
    Name = "birb-api-log-group"
  }
}

resource "aws_cloudwatch_log_stream" "birb_api_log_stream" {
  name           = "birb-api-log-stream"
  log_group_name = "${aws_cloudwatch_log_group.birb_api_log_group.name}"
}

# Set up cloudwatch group and log stream and retain logs for 30 days
resource "aws_cloudwatch_log_group" "birb_edgar_worker_log_group" {
  name              = "/ecs/birb-edgar-worker"
  retention_in_days = 30

  tags {
    Name = "birb-edgar-worker-log-group"
  }
}

resource "aws_cloudwatch_log_stream" "birb_edgar_worker_log_stream" {
  name           = "birb-edgar-worker-log-group"
  log_group_name = "${aws_cloudwatch_log_group.birb_edgar_worker_log_group.name}"
}
