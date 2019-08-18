resource "aws_s3_bucket" "birb_edgar_filings" {
  bucket = "birb-edgar-filings"
  acl    = "private"

  tags = {
    Name = "Edgar Filings"
  }

  // Allow GZIP compression in conjunction with CloudFront.
  // See: https://ithoughthecamewithyou.com/post/enable-gzip-compression-for-amazon-s3-hosted-website-in-cloudfront
  cors_rule {
    allowed_headers = ["Authorization", "Content-Length"]
    allowed_methods = ["GET"]
    allowed_origins = ["*"]
  }
}

// AWS_SDK_LOAD_CONFIG=1   AWS_PROFILE=birb terraform destroy -target=aws_s3_bucket_policy.birb_edgar_filings -var-file=terraform/production.secret.tfvars terraform/
// AWS_SDK_LOAD_CONFIG=1   AWS_PROFILE=birb terraform plan -out=plan -target=aws_s3_bucket.birb_edgar_filings -var-file=terraform/production.secret.tfvars terraform/
