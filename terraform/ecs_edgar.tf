data "template_file" "birb_edgar_worker_app" {
  template = "${file("terraform/templates/ecs/birb_edgar_worker.json.tpl")}"

  vars {
    repo_url         = "${aws_ecr_repository.birb_api_repo.repository_url}"
    app_name         = "birb-edgar"
    cpu              = "${var.birb_edgar_worker_cpu}"
    memory           = "${var.birb_edgar_worker_memory}"
    aws_region       = "${var.aws_region}"
    ROCKET_DATABASES = "${aws_secretsmanager_secret.ROCKET_DATABASES.arn}"
  }
}

# API Cluster
resource "aws_ecs_cluster" "birb-edgar-worker" {
  name = "birb-edgar-worker-cluster"
}

# API Service
resource "aws_ecs_service" "birb-edgar-worker" {
  name            = "birb-edgar-worker-service"
  cluster         = "${aws_ecs_cluster.birb-edgar-worker.id}"
  task_definition = "${aws_ecs_task_definition.birb-edgar-worker.arn}"
  desired_count   = "${var.app_count}"
  launch_type     = "EC2"

  network_configuration {
    security_groups  = ["${aws_security_group.ecs_task_workers.id}"]
    subnets          = ["${aws_subnet.private.*.id}"]
    assign_public_ip = false
  }
}

# API Task Definition
resource "aws_ecs_task_definition" "birb-edgar-worker" {
  family                   = "birb-edgar-worker-task"
  execution_role_arn       = "${aws_iam_role.ecs-instance-role.arn}"
  network_mode             = "awsvpc"
  requires_compatibilities = ["EC2"]
  cpu                      = "${var.birb_api_cpu}"
  memory                   = "${var.birb_api_memory}"
  container_definitions    = "${data.template_file.birb_api_app.rendered}"
}

#
# the ECS optimized AMI's change by region. You can lookup the AMI here:
# https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-optimized_AMI.html
#
# us-east-1 ami-aff65ad2
# us-east-2 ami-64300001
# us-west-1 ami-69677709
# us-west-2 ami-40ddb938
#

#
# need to add security group config
# so that we can ssh into an ecs host from bastion box
#

resource "aws_launch_configuration" "ecs-launch-configuration" {
  name                 = "ecs-launch-configuration"
  image_id             = "ami-0bc08634af113cccb"
  instance_type        = "t2.medium"
  iam_instance_profile = "${aws_iam_instance_profile.ecs-instance-profile.id}"

  root_block_device {
    volume_type           = "standard"
    volume_size           = 100
    delete_on_termination = true
  }

  lifecycle {
    create_before_destroy = true
  }

  associate_public_ip_address = false

  # register the cluster name with ecs-agent which will in turn coord
  # with the AWS api about the cluster
  user_data = <<EOF
#!/bin/bash
echo ECS_CLUSTER=birb-edgar-worker-cluster >> /etc/ecs/ecs.config
EOF
}

# need an ASG so we can easily add more ecs host nodes as necessary
resource "aws_autoscaling_group" "ecs-autoscaling-group" {
  name             = "ecs-autoscaling-group"
  max_size         = "2"
  min_size         = "1"
  desired_capacity = "1"

  # vpc_zone_identifier = ["subnet-41395d29"]
  vpc_zone_identifier  = ["${aws_subnet.private.*.id}"]
  launch_configuration = "${aws_launch_configuration.ecs-launch-configuration.name}"
  health_check_type    = "ELB"

  tag {
    key                 = "Name"
    value               = "ECS-${aws_ecs_cluster.birb-edgar-worker.name}"
    propagate_at_launch = true
  }
}
