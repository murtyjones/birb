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

data "aws_iam_policy_document" "birb_edgar_filings_s3_policy" {
  statement {
    sid       = "OnlyCloudfrontReadAccess"
    actions   = ["s3:GetObject"]
    resources = ["${aws_s3_bucket.birb_edgar_filings.arn}/*"]

    principals {
      type        = "AWS"
      identifiers = ["${aws_cloudfront_origin_access_identity.birb_raw_filings_access_identity.iam_arn}"]
    }
  }
}

resource "aws_s3_bucket_policy" "birb_edgar_filings" {
  bucket = "${aws_s3_bucket.birb_edgar_filings.id}"
  policy = "${data.aws_iam_policy_document.birb_edgar_filings_s3_policy.json}"

}

// AWS_SDK_LOAD_CONFIG=1   AWS_PROFILE=birb terraform plan -target=aws_s3_bucket_policy.birb_edgar_filings -var-file=terraform/production.secret.tfvars -out=plan terraform/
