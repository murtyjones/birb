data "template_file" "birb_api_app" {
  template = "${file("terraform/templates/ecs/birb_api_app.json.tpl")}"

  vars {
    repo_url         = "${aws_ecr_repository.api_repo.repository_url}"
    app_name         = "birb-api"
    birb_api_cpu     = "${var.birb_api_cpu}"
    birb_api_memory  = "${var.birb_api_memory}"
    aws_region       = "${var.aws_region}"
    app_port         = "${var.app_port}"
    ROCKET_DATABASES = "${aws_secretsmanager_secret.ROCKET_DATABASES.arn}"
  }
}

# API Cluster
resource "aws_ecs_cluster" "api_cluster" {
  name = "birb-api-cluster"
}

# API Service
resource "aws_ecs_service" "api_service" {
  name            = "birb-api-service"
  cluster         = "${aws_ecs_cluster.api_cluster.id}"
  task_definition = "${aws_ecs_task_definition.api_task.arn}"
  desired_count   = "${var.app_count}"
  launch_type     = "FARGATE"

  network_configuration {
    security_groups  = ["${aws_security_group.ecs_tasks.id}"]
    subnets          = ["${aws_subnet.private.*.id}"]
    assign_public_ip = true
  }

  load_balancer {
    target_group_arn = "${aws_alb_target_group.api_target_group.id}"
    container_name   = "birb-api"
    container_port   = "${var.app_port}"
  }

  depends_on = [
    "aws_alb_listener.api_lb_listener",
  ]
}

# API Task Definition
resource "aws_ecs_task_definition" "api_task" {
  family                   = "birb-api-task"
  execution_role_arn       = "${aws_iam_role.task_execution_role.arn}"
  network_mode             = "awsvpc"
  requires_compatibilities = ["FARGATE"]
  cpu                      = "${var.birb_api_cpu}"
  memory                   = "${var.birb_api_memory}"
  container_definitions    = "${data.template_file.birb_api_app.rendered}"
}
