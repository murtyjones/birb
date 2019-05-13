data "template_file" "birb_edgar_worker_app" {
  template = "${file("terraform/templates/ecs/birb_edgar_worker.json.tpl")}"

  vars {
    repo_url     = "${aws_ecr_repository.birb_edgar_worker_repo.repository_url}"
    app_name     = "birb-edgar"
    cpu          = "${var.birb_edgar_worker_cpu}"
    memory       = "${var.birb_edgar_worker_memory}"
    aws_region   = "${var.aws_region}"
    DATABASE_URI = "${aws_secretsmanager_secret.DATABASE_URI.arn}"
  }
}

# the ECS optimized AMI's change by region. You can lookup the AMI here:
# https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-optimized_AMI.html
#
# us-east-1 ami-aff65ad2
# us-east-2 ami-64300001
# us-west-1 ami-69677709
# us-west-2 ami-40ddb938

# need to add security group config
# so that we can ssh into an ecs host from bastion box

resource "aws_launch_configuration" "ecs-launch-configuration" {
  name                 = "ecs-launch-configuration"
  image_id             = "ami-0bc08634af113cccb"
  instance_type        = "t2.micro"
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
echo ECS_CLUSTER=birb-edgar-cluster >> /etc/ecs/ecs.config
EOF
}

# need an ASG so we can easily add more ecs host nodes as necessary
resource "aws_autoscaling_group" "ecs-autoscaling-group" {
  name             = "ecs-autoscaling-group"
  max_size         = "2"
  min_size         = "1"
  desired_capacity = "1"

  vpc_zone_identifier  = ["${aws_subnet.private.*.id}"]
  launch_configuration = "${aws_launch_configuration.ecs-launch-configuration.name}"
  health_check_type    = "ELB"

  tag {
    key                 = "Name"
    value               = "ECS-birb-edgar-cluster"
    propagate_at_launch = true
  }
}

resource "aws_ecs_cluster" "birb-edgar-cluster" {
  name = "birb-edgar-cluster"
}

resource "aws_ecs_task_definition" "birb-edgar-task" {
  family                = "birb-edgar-worker-task"
  execution_role_arn    = "${aws_iam_role.ecs-instance-role.arn}"
  container_definitions = "${data.template_file.birb_edgar_worker_app.rendered}"
}

resource "aws_ecs_service" "birb-edgar-service" {
  name            = "birb-edgar-service"
  cluster         = "${aws_ecs_cluster.birb-edgar-cluster.id}"
  task_definition = "${aws_ecs_task_definition.birb-edgar-task.arn}"
  desired_count   = 1
}
