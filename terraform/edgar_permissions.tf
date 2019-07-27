resource "aws_iam_role" "edgar_instance_role" {
  name               = "edgar_instance_role"
  path               = "/"
  assume_role_policy = data.aws_iam_policy_document.ecs-instance-policy.json
}

data "aws_iam_policy_document" "ecs-instance-policy" {
  statement {
    actions = ["sts:AssumeRole"]

    principals {
      type = "Service"

      identifiers = [
        "ec2.amazonaws.com",
        "ecs-tasks.amazonaws.com",
      ]
    }
  }
}

resource "aws_iam_policy" "edgar_resource_access_policy" {
  name = "edgar_resource_access_policy"
  path = "/"

  policy = <<EOF
{
    "Version": "2012-10-17",
    "Statement": [
        {
            "Effect": "Allow",
            "Action": [
                "cloudwatch:DescribeAlarms",
                "cloudwatch:PutMetricAlarm",
                "logs:CreateLogStream",
                "logs:CreateLogGroup",
                "logs:PutLogEvents"
            ],
            "Resource": [
                "${aws_cloudwatch_log_group.edgar_log_group.arn}",
                "${aws_cloudwatch_log_stream.edgar_log_stream.arn}"
            ]
        },
        {
            "Effect": "Allow",
            "Action": [
                "secretsmanager:GetSecretValue"
            ],
            "Resource": [
                "${aws_secretsmanager_secret.DATABASE_URI.arn}"
            ]
        },
        {
            "Effect": "Allow",
            "Action": [
                "s3:GetObject",
                "s3:PutObject"
            ],
            "Resource": [
                "${aws_s3_bucket.birb_edgar_filings.arn}/*",
                "${aws_s3_bucket.birb_edgar_indexes.arn}/*"
            ]
        }
    ]
}
EOF

}

resource "aws_iam_role_policy_attachment" "edgar_instance_role_attachment" {
  role = aws_iam_role.edgar_instance_role.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AmazonEC2ContainerServiceforEC2Role"
}

resource "aws_iam_role_policy_attachment" "edgar_resource_access_attachment" {
  role = aws_iam_role.edgar_instance_role.name
  policy_arn = aws_iam_policy.edgar_resource_access_policy.arn
}

resource "aws_iam_instance_profile" "edgar_instance_profile" {
  name = "edgar_instance_profile"
  path = "/"
  role = aws_iam_role.edgar_instance_role.id

  provisioner "local-exec" {
    command = "sleep 60"
  }
}

resource "aws_iam_role" "edgar_service_role" {
  name = "edgar_service_role"
  path = "/"
  assume_role_policy = data.aws_iam_policy_document.edgar_service_policy.json
}

resource "aws_iam_role_policy_attachment" "edgar_service_role_attachment" {
  role = aws_iam_role.edgar_service_role.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AmazonEC2ContainerServiceRole"
}

data "aws_iam_policy_document" "edgar_service_policy" {
  statement {
    actions = ["sts:AssumeRole"]

    principals {
      type = "Service"
      identifiers = ["ecs.amazonaws.com"]
    }
  }
}

