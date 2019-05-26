data "template_file" "birb_server_app" {
  template = "${file("terraform/templates/ecs/birb_server_app.json.tpl")}"

  vars {
    repo_url         = "${aws_ecr_repository.server_repo.repository_url}"
    app_name         = "birb-server"
    birb_server_cpu     = "${var.birb_server_cpu}"
    birb_server_memory  = "${var.birb_server_memory}"
    aws_region       = "${var.aws_region}"
    app_port         = "${var.app_port}"
    ROCKET_DATABASES = "${aws_secretsmanager_secret.ROCKET_DATABASES.arn}"
  }
}

# API Cluster
resource "aws_ecs_cluster" "server_cluster" {
  name = "birb-server-cluster"
}

# API Service
resource "aws_ecs_service" "server_service" {
  name            = "birb-server-service"
  cluster         = "${aws_ecs_cluster.server_cluster.id}"
  task_definition = "${aws_ecs_task_definition.server_task.arn}"
  desired_count   = "${var.app_count}"
  launch_type     = "FARGATE"

  network_configuration {
    security_groups  = ["${aws_security_group.ecs_tasks.id}"]
    subnets          = ["${aws_subnet.private.*.id}"]
    assign_public_ip = true
  }

  load_balancer {
    target_group_arn = "${aws_alb_target_group.server_target_group.id}"
    container_name   = "birb-server"
    container_port   = "${var.app_port}"
  }

  depends_on = [
    "aws_alb_listener.server_lb_listener",
  ]
}

# API Task Definition
resource "aws_ecs_task_definition" "server_task" {
  family                   = "birb-server-task"
  execution_role_arn       = "${aws_iam_role.task_execution_role.arn}"
  network_mode             = "awsvpc"
  requires_compatibilities = ["FARGATE"]
  cpu                      = "${var.birb_server_cpu}"
  memory                   = "${var.birb_server_memory}"
  container_definitions    = "${data.template_file.birb_server_app.rendered}"
}
