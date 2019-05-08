resource "aws_ecs_cluster" "edgar-worker" {
  name = "edgar-worker-cluster"
}

data "template_file" "birb_edgar_worker" {
  template = "${file("terraform/templates/ecs/birb_edgar_worker.json.tpl")}"

  vars {
    repo_url         = "${aws_ecr_repository.birb_edgar_worker_repo.repository_url}"
    app_name         = "birb-edgar-worker"
    cpu              = "${var.birb_edgar_worker_cpu}"
    memory           = "${var.birb_edgar_worker_memory}"
    aws_region       = "${var.aws_region}"
    ROCKET_DATABASES = "${aws_secretsmanager_secret.ROCKET_DATABASES.arn}"
  }
}

resource "aws_ecs_task_definition" "edgar-worker" {
  family                   = "birb-edgar-worker-app-task"
  execution_role_arn       = "${aws_iam_role.task_execution_role.arn}"
  network_mode             = "awsvpc"
  cpu                      = "${var.birb_edgar_worker_cpu}"
  memory                   = "${var.birb_edgar_worker_memory}"
  container_definitions    = "${data.template_file.birb_edgar_worker.rendered}"
}

resource "aws_ecs_service" "edgar-worker" {
  name            = "birb-edgar-worker-service"
  cluster         = "${aws_ecs_cluster.edgar-worker.id}"
  task_definition = "${aws_ecs_task_definition.edgar-worker.arn}"
  desired_count   = "${var.app_count}"

  network_configuration {
    security_groups  = ["${aws_security_group.ecs_tasks.id}"]
    subnets          = ["${aws_subnet.private.*.id}"]
    assign_public_ip = false
  }
}
