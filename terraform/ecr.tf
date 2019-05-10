resource "aws_ecr_repository" "birb_api_repo" {
  name = "birb_api"
}

resource "aws_ecr_repository" "birb_edgar_worker_repo" {
  name = "birb_edgar_worker"
}
