resource "aws_ecr_repository" "server_repo" {
  name = "birb_server"
}

resource "aws_ecr_repository" "edgar_repo" {
  name = "birb_edgar_worker"
}
