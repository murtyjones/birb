resource "aws_iam_instance_profile" "ecs" {
  name  = "birb-edgar-worker-ecs-instance"
  role = "${aws_iam_role.ecs_instance.name}"
}

resource "aws_iam_policy_attachment" "ecs_instance" {
  name       = "birb-edgar-worker-ecs-instance"
  roles      = ["${aws_iam_role.ecs_instance.name}"]
  policy_arn = "arn:aws:iam::aws:policy/service-role/AmazonEC2ContainerServiceforEC2Role"
}

resource "aws_iam_role" "ecs_instance" {
  name = "birb-edgar-worker-ecs-instance"
  path = "/"

  assume_role_policy = <<EOF
{
    "Version": "2008-10-17",
    "Statement": [
      {
        "Action": "sts:AssumeRole",
        "Principal": {
          "Service": "ec2.amazonaws.com"
        },
        "Effect": "Allow",
        "Sid": ""
      }
    ]
}
EOF
}

resource "aws_security_group" "ecs_instance" {
  name        = "birb-edgar-worker-ecs-instance"
  description = "container security group for birb-edgar-worker"
  vpc_id      = "${aws_vpc.main.id}"

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }
}

resource "aws_iam_policy_attachment" "fleet" {
  name       = "birb-edgar-worker-fleet"
  roles      = ["${aws_iam_role.fleet.name}"]
  policy_arn = "arn:aws:iam::aws:policy/service-role/AmazonEC2SpotFleetRole"
}

resource "aws_iam_role" "fleet" {
  name = "birb-edgar-worker-fleet"

  assume_role_policy = <<EOF
{
  "Version": "2008-10-17",
  "Statement": [
    {
      "Sid": "",
      "Effect": "Allow",
      "Principal": {
        "Service": [
          "spotfleet.amazonaws.com",
          "ec2.amazonaws.com"
        ]
      },
      "Action": "sts:AssumeRole"
    }
  ]
}
EOF
}

resource "aws_spot_fleet_request" "main" {
  iam_fleet_role                      = "${aws_iam_role.fleet.arn}"
//  spot_price                          = "${var.spot_prices[0]}"
  allocation_strategy                 = "${var.strategy}"
  target_capacity                     = "${var.instance_count}"
  terminate_instances_with_expiration = true
  valid_until                         = "${var.valid_until}"

  launch_specification {
    ami                    = "${var.ami}"
    instance_type          = "${var.instance_type}"
//    spot_price             = "${var.spot_prices[0]}"
    subnet_id              = "${aws_subnet.private.0.id}"
    vpc_security_group_ids = ["${aws_security_group.ecs_instance.id}"]
    iam_instance_profile   = "${aws_iam_instance_profile.ecs.name}"

    root_block_device = {
      volume_type = "gp2"
      volume_size = "${var.volume_size}"
    }

    user_data = <<USER_DATA
#!/bin/bash
echo ECS_CLUSTER=${aws_ecs_cluster.edgar-worker.name} >> /etc/ecs/ecs.config
USER_DATA
  }

  depends_on = ["aws_iam_policy_attachment.fleet"]
}
