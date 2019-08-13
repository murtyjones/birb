data "template_file" "birb_edgar_app" {
  template = file("terraform/templates/ecs/birb_edgar_worker.json.tpl")

  vars = {
    repo_url     = aws_ecr_repository.edgar_repo.repository_url
    app_name     = "birb-edgar"
    cpu          = var.birb_edgar_worker_cpu
    memory       = var.birb_edgar_worker_memory
    aws_region   = var.aws_region
    DATABASE_URI = aws_secretsmanager_secret.DATABASE_URI.arn
  }
}

# the ECS optimized AMI's change by region. You can lookup the AMI here:
# https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-optimized_AMI.html
# e.g.
# us-east-1 ami-aff65ad2
# us-east-2 ami-64300001
# us-west-1 ami-69677709
# us-west-2 ami-40ddb938

# need to add security group config
# so that we can ssh into an ecs host from bastion box

resource "aws_launch_configuration" "edgar_launch_configuration" {
  name_prefix          = "edgar-launch-config-"
  image_id             = "ami-0bc08634af113cccb"
  instance_type        = "t3.small"
  iam_instance_profile = aws_iam_instance_profile.edgar_instance_profile.id

  security_groups = [
    aws_security_group.birb-edgar.id,
  ]

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
echo ECS_CLUSTER=edgar_cluster >> /etc/ecs/ecs.config
EOF

}

# need an ASG so we can easily add more ecs host nodes as necessary
resource "aws_autoscaling_group" "edgar_autoscaling" {
  name             = "edgar_autoscaling"
  max_size         = "1"
  min_size         = "1"
  desired_capacity = "1"

  vpc_zone_identifier  = aws_subnet.private.*.id
  launch_configuration = aws_launch_configuration.edgar_launch_configuration.name
  health_check_type    = "ELB"

  tag {
    key                 = "Name"
    value               = "ECS-edgar_cluster"
    propagate_at_launch = true
  }
}

resource "aws_ecs_cluster" "edgar_cluster" {
  name = "edgar_cluster"
}

resource "aws_ecs_task_definition" "edgar_task" {
  family                = "birb-edgar-task"
  execution_role_arn    = aws_iam_role.edgar_instance_role.arn
  container_definitions = data.template_file.birb_edgar_app.rendered
}

resource "aws_ecs_service" "edgar_service" {
  name            = "edgar_service"
  cluster         = aws_ecs_cluster.edgar_cluster.id
  task_definition = aws_ecs_task_definition.edgar_task.arn
  desired_count   = 1
}

