# Certificate lives seperately from the rest of the terraform infrastructure
# so that AWS's certificate issuance limit (20) isn't exhausted as we tear
# down and rebuild the infrastructure at will.

# When provisioning:

# ...run this first:
resource "aws_acm_certificate" "api" {
  domain_name       = "api.${var.domain}"
  validation_method = "DNS"

  lifecycle {
    create_before_destroy = true
  }
}

data "aws_route53_zone" "api" {
  name = "${var.domain}."
}

# ...then this:
resource "aws_route53_record" "api_validation" {
  depends_on = ["aws_acm_certificate.api"]
  name       = "${lookup(aws_acm_certificate.api.domain_validation_options[0], "resource_record_name")}"
  type       = "${lookup(aws_acm_certificate.api.domain_validation_options[0], "resource_record_type")}"
  zone_id    = "${data.aws_route53_zone.api.zone_id}"
  records    = ["${lookup(aws_acm_certificate.api.domain_validation_options[0], "resource_record_value")}"]
  ttl        = 300
}

# ...then this:
resource "aws_acm_certificate_validation" "api" {
  certificate_arn         = "${aws_acm_certificate.api.arn}"
  validation_record_fqdns = ["${aws_route53_record.api_validation.*.fqdn}"]
}