resource "aws_ecr_repository" "birb-repo" {
  name = "${var.api_image}"
}
