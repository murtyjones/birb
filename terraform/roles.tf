resource "aws_iam_role" "autoscale_role" {
  name = "fargate-autoscale-role"

  assume_role_policy = <<EOF
{
            "Version": "2012-10-17",
            "Statement": [
                {
                    "Effect": "Allow",
                    "Principal": {
                        "Service": "application-autoscaling.amazonaws.com"
                    },
                    "Action": "sts:AssumeRole"
                }
            ]
        }
EOF

}

resource "aws_iam_policy" "autoscale_policy" {
  name = "fargate-autoscale-policy"
  path = "/"

  policy = <<EOF
{
    "Version": "2012-10-17",
    "Statement": [
        {
            "Effect": "Allow",
            "Action": [
                "ecs:DescribeServices",
                "ecs:UpdateService"
            ],
            "Resource": [
                "${aws_ecs_cluster.server_cluster.arn}"
            ]
        },
        {
            "Effect": "Allow",
            "Action": [
                "cloudwatch:DescribeAlarms",
                "cloudwatch:PutMetricAlarm"
            ],
            "Resource": [
                "${aws_ecs_cluster.server_cluster.arn}"
            ]
        }
    ]
}
EOF

}

resource "aws_iam_role_policy_attachment" "server_autoscale_attachment" {
depends_on = [aws_iam_role.autoscale_role]
role       = aws_iam_role.autoscale_role.name
policy_arn = aws_iam_policy.autoscale_policy.arn
}

resource "aws_iam_role" "task_execution_role" {
name = "fargate-task-execution-role"

assume_role_policy = <<EOF
{
    "Version": "2012-10-17",
    "Statement": [
        {
            "Sid": "",
            "Effect": "Allow",
            "Principal": {
                "Service": "ecs-tasks.amazonaws.com"
            },
            "Action": "sts:AssumeRole"
        }
    ]
}
EOF

}

resource "aws_iam_policy" "task_execution_policy" {
name = "fargate-task-execution-policy"
path = "/"

policy = <<EOF
{
    "Version": "2012-10-17",
    "Statement": [
        {
            "Effect": "Allow",
            "Action": [
                "ecs:DescribeServices",
                "ecs:UpdateService"
            ],
            "Resource": [
                "${aws_ecs_cluster.server_cluster.arn}"
            ]
        },
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
                "${aws_cloudwatch_log_group.server_log_group.arn}",
                "${aws_cloudwatch_log_stream.server_log_stream.arn}"
            ]
        },
        {
            "Effect": "Allow",
            "Action": [
                "ecr:BatchGetImage",
                "ecr:GetAuthorizationToken",
                "ecr:GetDownloadUrlForLayer"
            ],
            "Resource": [
                "*"
            ]
        },
        {
              "Effect": "Allow",
              "Action": "secretsmanager:GetSecretValue",
              "Resource": [
                  "${aws_secretsmanager_secret.ROCKET_DATABASES.arn}"
              ]
        }
    ]
}
EOF

}

resource "aws_iam_role_policy_attachment" "server_execution_attachment" {
  depends_on = [aws_iam_role.task_execution_role]
  role       = aws_iam_role.task_execution_role.name
  policy_arn = aws_iam_policy.task_execution_policy.arn
}

