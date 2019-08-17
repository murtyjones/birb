resource "aws_cloudfront_origin_access_identity" "birb_raw_filings_access_identity" {
  comment = "Restrict bucket access to those with signed URLs"
}

// This Route53 record will point at our CloudFront distribution for raw-filings.birb.io.
resource "aws_cloudfront_distribution" "birb_raw_filings_cdn" {
  // origin is where CloudFront gets its content from.
  origin {
    // Here we're using our S3 bucket's URL!
    domain_name = "${aws_s3_bucket.birb_edgar_filings.bucket_regional_domain_name}"
    // This can be any name to identify this origin.
    origin_id = "${var.raw_filings_domain_name}"

    s3_origin_config {
      origin_access_identity = "${aws_cloudfront_origin_access_identity.birb_raw_filings_access_identity.cloudfront_access_identity_path}"
    }
  }

  enabled             = true
  default_root_object = "index.html"

  default_cache_behavior {
    viewer_protocol_policy = "redirect-to-https"
    compress               = true
    allowed_methods        = ["GET", "HEAD"]
    cached_methods         = ["GET", "HEAD"]
    // This needs to match the `origin_id` above.
    target_origin_id = "${var.raw_filings_domain_name}"
    min_ttl          = 0
    default_ttl      = 86400
    max_ttl          = 31536000

    forwarded_values {
      query_string = false
      cookies {
        forward = "none"
      }
    }
  }

  // Here we're ensuring we can hit this distribution using raw-filings.birb.io
  // rather than the domain name CloudFront gives us.
  aliases = [
    "${var.raw_filings_domain_name}"
  ]

  restrictions {
    geo_restriction {
      restriction_type = "none"
    }
  }

  // Here's where our certificate is loaded in!
  viewer_certificate {
    acm_certificate_arn = "${var.birb_raw_filings_certificate_arn}"
    ssl_support_method  = "sni-only"
  }
}

//AWS_SDK_LOAD_CONFIG=1   AWS_PROFILE=birb terraform plan -target=aws_route53_record.birb_raw_filings_cdn -target=aws_cloudfront_distribution.birb_raw_filings_cdn -var-file=terraform/production.secret.tfvars -out=plan terraform/
