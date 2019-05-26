data "aws_route53_zone" "birb" {
  zone_id = "Z1D0S9UH91H2SA"
}

resource "aws_route53_record" "birb" {
  zone_id = "${data.aws_route53_zone.birb.zone_id}"
  name    = "birb.io"
  type    = "A"

  alias {
    name                   = "${aws_alb.server_load_balancer.dns_name}"
    zone_id                = "${aws_alb.server_load_balancer.zone_id}"
    evaluate_target_health = true
  }
}
