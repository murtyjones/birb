resource "aws_ecr_repository" "birb_repo" {
  name = "${var.app_name}"
}
