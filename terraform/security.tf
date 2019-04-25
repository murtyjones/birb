# ALB Security Group: Edit this to restrict access to the application
resource "aws_security_group" "lb" {
  name        = "birb-api-load-balancer-security-group"
  description = "Allow access on port 443 only to ALB"
  vpc_id      = "${aws_vpc.main.id}"

  ingress {
    protocol    = "tcp"
    from_port   = 443
    to_port     = 443
    cidr_blocks = ["0.0.0.0/0"]
  }

  egress {
    protocol    = "-1"
    from_port   = 0
    to_port     = 0
    cidr_blocks = ["0.0.0.0/0"]
  }
}

# Traffic to the ECS cluster should only come from the ALB
resource "aws_security_group" "ecs_tasks" {
  name        = "birb-api-ecs-tasks-security-group"
  description = "allow inbound access from the ALB only"
  vpc_id      = "${aws_vpc.main.id}"

  ingress {
    protocol        = "tcp"
    from_port       = "${var.app_port}"
    to_port         = "${var.app_port}"
    security_groups = ["${aws_security_group.lb.id}"]
  }

  egress {
    protocol    = "-1"
    from_port   = 0
    to_port     = 0
    cidr_blocks = ["0.0.0.0/0"]
  }
}

# ECS/Bastion access to RDS
resource "aws_security_group" "birb_rds" {
  name        = "birb-rds"
  description = "specify inbound access rules"
  vpc_id      = "${aws_vpc.main.id}"

  ingress {
    protocol        = "tcp"
    from_port       = "5432"
    to_port         = "5432"
    security_groups = [
      # Allow ECS tasks to access RDS
      "${aws_security_group.ecs_tasks.id}",
      # Allow the bastion to access RDS
      "${aws_security_group.bastion.id}"
    ]
  }

  egress {
    protocol    = "-1"
    from_port   = 0
    to_port     = 0
    cidr_blocks = ["0.0.0.0/0"]
  }
}

resource "aws_security_group" "bastion" {
  name   = "bastion-security-group"
  vpc_id = "${aws_vpc.main.id}"

  ingress {
    protocol    = "tcp"
    from_port   = 22
    to_port     = 22
    cidr_blocks = [
      # as an extra layer of security, only allow access from these IPS:
      # Marty:
      "${var.marty_ip_address_1}/32",
      "${var.marty_ip_address_2}/32"
    ]
  }

  egress {
    protocol    = "-1"
    from_port   = 0
    to_port     = 0
    cidr_blocks = ["0.0.0.0/0"]
  }
}