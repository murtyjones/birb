resource "aws_ecr_repository" "api_repo" {
  name = "birb_api"
}

resource "aws_ecr_repository" "edgar_repo" {
  name = "birb_edgar_worker"
}
