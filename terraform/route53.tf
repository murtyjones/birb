data "aws_route53_zone" "birb" {
  zone_id = "Z3E84F61Q1N2YA"
}

// Create a variable for our domain name because we'll be using it a lot.
variable "www_domain_name" {
  default = "www.birb.io"
}

// We'll also need the root domain (also known as zone apex or naked domain).
variable "root_domain_name" {
  default = "birb.io"
}

// Here's our CDN for filings
variable "filings_domain_name" {
  default = "filings.birb.io"
}

// Here's our doc site
variable "docs_domain_name" {
  default = "docs.birb.io"
}

// This Route53 record will point at our CloudFront distribution for birb.io.
resource "aws_route53_record" "birb_root" {
  zone_id = "${data.aws_route53_zone.birb.zone_id}"

  // NOTE: name is intentionally blank here.
  name = ""
  type = "A"

  alias {
    name                   = "${aws_cloudfront_distribution.birb_www_distribution.domain_name}"
    zone_id                = "${aws_cloudfront_distribution.birb_www_distribution.hosted_zone_id}"
    evaluate_target_health = false
  }
}

// This Route53 record will point at our CloudFront distribution for api.birb.io.
resource "aws_route53_record" "birb" {
  zone_id = data.aws_route53_zone.birb.zone_id
  name    = "api.birb.io"
  type    = "A"

  alias {
    name                   = aws_alb.server_load_balancer.dns_name
    zone_id                = aws_alb.server_load_balancer.zone_id
    evaluate_target_health = true
  }
}

// This Route53 record will point at our CloudFront distribution for www.birb.io.
resource "aws_route53_record" "birb_www" {
  zone_id = "${data.aws_route53_zone.birb.zone_id}"

  name = "${var.www_domain_name}"
  type = "A"

  alias {
    name                   = "${aws_cloudfront_distribution.birb_www_distribution.domain_name}"
    zone_id                = "${aws_cloudfront_distribution.birb_www_distribution.hosted_zone_id}"
    evaluate_target_health = false
  }
}

// This Route53 record will point at our CloudFront distribution for docs.birb.io.
resource "aws_route53_record" "birb_docs" {
  zone_id = "${data.aws_route53_zone.birb.zone_id}"

  name = "${var.docs_domain_name}"
  type = "A"

  alias {
    name                   = "${aws_cloudfront_distribution.birb_docs_distribution.domain_name}"
    zone_id                = "${aws_cloudfront_distribution.birb_docs_distribution.hosted_zone_id}"
    evaluate_target_health = false
  }
}


