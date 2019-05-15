resource "aws_alb" "api_load_balancer" {
  name            = "api-load-balancer"
  subnets         = ["${aws_subnet.public.*.id}"]
  security_groups = ["${aws_security_group.lb.id}"]
}

resource "aws_alb_target_group" "api_target_group" {
  name        = "api-target-group"
  port        = 80
  protocol    = "HTTP"
  vpc_id      = "${aws_vpc.main.id}"
  target_type = "ip"

  health_check {
    healthy_threshold   = "3"
    interval            = "30"
    protocol            = "HTTP"
    matcher             = "200"
    timeout             = "3"
    path                = "${var.health_check_path}"
    unhealthy_threshold = "2"
  }
}

# Redirect all traffic from the ALB to the target group
resource "aws_alb_listener" "api_lb_listener" {
  load_balancer_arn = "${aws_alb.api_load_balancer.id}"
  port              = "443"
  protocol          = "HTTPS"
  ssl_policy        = "ELBSecurityPolicy-2016-08"
  certificate_arn   = "${var.birb_api_certificate_arn}"

  default_action {
    target_group_arn = "${aws_alb_target_group.api_target_group.id}"
    type             = "forward"
  }
}
