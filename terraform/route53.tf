data "aws_route53_zone" "birb" {
    zone_id = "Z1D0S9UH91H2SA"
}

resource "aws_route53_record" "birb" {
  zone_id = "${data.aws_route53_zone.birb.zone_id}"
  name    = "api.birb.io"
  type    = "A"

  alias {
    name                   = "${aws_alb.main.dns_name}"
    zone_id                = "${aws_alb.main.zone_id}"
    evaluate_target_health = true
  }
}
