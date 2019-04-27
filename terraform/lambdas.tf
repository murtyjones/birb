resource "aws_lambda_function" "edgar_worker" {
  filename         = "edgar_worker_payload.zip"
  function_name    = "edgar_worker"
  role             = "${aws_iam_role.edgar_worker.arn}"
  handler          = "main"
  # The filebase64sha256() function is available in Terraform 0.11.12 and later
  # For Terraform 0.11.11 and earlier, use the base64sha256() function and the file() function:
  # source_code_hash = "${base64sha256(file("lambda_function_payload.zip"))}"
  source_code_hash = "${filebase64sha256(file("${path.module}/in/edgar_worker_payload.zip"))}"
  runtime          = "provided"

  environment {
    variables = {
      foo = "bar"
    }
  }
}