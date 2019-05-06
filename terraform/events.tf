resource "aws_cloudwatch_event_rule" "edgar_worker_trigger" {
  name = "edgar_worker_trigger"
  description = "Fires every minute"
  schedule_expression = "rate(1 minute)"
}

resource "aws_cloudwatch_event_target" "edgar_worker_trigger_five_minutes" {
  rule = "${aws_cloudwatch_event_rule.edgar_worker_trigger.name}"
  target_id = "check_foo"
  arn = "${aws_lambda_function.edgar_worker.arn}"
}
