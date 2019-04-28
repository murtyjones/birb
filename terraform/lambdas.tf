resource "aws_lambda_function" "edgar_worker" {
  filename      = "out.zip"
  function_name = "edgar_worker"
  role          = "${aws_iam_role.edgar_worker.arn}"
  handler       = "out/edgar_worker"

  # The filebase64sha256() function is available in Terraform 0.11.12 and later
  # For Terraform 0.11.11 and earlier, use the base64sha256() function and the file() function:
  # source_code_hash = "${base64sha256(file("lambda_function_payload.zip"))}"
  source_code_hash = "${filebase64sha256("out.zip")}"

  runtime = "provided"

  // TODO fix deployment errors around this VPC so that DB access
  // is available to the lambda:
  //  vpc_config {
  //    security_group_ids = ["${aws_security_group.birb_rds.id}"]
  //    subnet_ids = ["${aws_subnet.public.*.id}"]
  //  }

  environment {
    variables = {
      foo = "bar"
    }
  }
}
