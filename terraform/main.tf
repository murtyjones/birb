resource "aws_ecr_repository" "birb-repo" {
  name = "${var.app_name}"
}
