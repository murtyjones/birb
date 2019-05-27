# ALB allows access from anywhere and can call anywhere (in pratice, only goes to ECS) tasks
resource "aws_security_group" "lb" {
  # Technically should be `birb-server-ecs-tasks-security-group` but hard to replace oh well
  name        = "birb-api-load-balancer-security-group"
  description = "Allow access on port 443 only to ALB"
  vpc_id      = "${aws_vpc.main.id}"

  ingress {
    protocol    = "tcp"
    from_port   = 443
    to_port     = 443
    cidr_blocks = ["0.0.0.0/0"]
  }

  ingress {
    protocol    = "tcp"
    from_port   = 80
    to_port     = 80
    cidr_blocks = ["0.0.0.0/0"]
  }

  egress {
    protocol    = "-1"
    from_port   = 0
    to_port     = 0
    cidr_blocks = ["0.0.0.0/0"]
  }
}

# Traffic to the ECS cluster should only come from the ALB; can call out to anywhere
resource "aws_security_group" "ecs_tasks" {
  # Technically should be `birb-server-ecs-tasks-security-group` but hard to replace oh well
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

# ECS/Bastion accessible from approved IPs (w/ SSH), able to call out to anywhere (including RDS)
resource "aws_security_group" "rds_security_group" {
  name        = "birb-rds"
  description = "specify inbound access rules"
  vpc_id      = "${aws_vpc.main.id}"

  ingress {
    protocol  = "tcp"
    from_port = "5432"
    to_port   = "5432"

    security_groups = [
      # Allow ECS tasks to access RDS
      "${aws_security_group.ecs_tasks.id}",

      # Allow the bastion to access RDS
      "${aws_security_group.bastion.id}",

      # Allow lambdas to access RDS
      "${aws_security_group.birb-edgar.id}",
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
    protocol  = "tcp"
    from_port = 22
    to_port   = 22

    cidr_blocks = ["0.0.0.0/0"]

    //    cidr_blocks = [
    //      # as an extra layer of security, only allow access from these IPS:
    //      # Marty:
    //       "${var.marty_ip_address_1}/32",
    //
    //       "${var.marty_ip_address_2}/32",
    //
    //       "${var.marty_ip_address_3}/32",
    //    ]
  }

  egress {
    protocol    = "-1"
    from_port   = 0
    to_port     = 0
    cidr_blocks = ["0.0.0.0/0"]
  }
}

# Lambdas will not be accessible from the internet, but able to make any outbound calls
resource "aws_security_group" "birb-edgar" {
  name        = "birb-edgar-security-group"
  description = "no inbound, only outbound"
  vpc_id      = "${aws_vpc.main.id}"

  egress {
    protocol    = "-1"
    from_port   = 0
    to_port     = 0
    cidr_blocks = ["0.0.0.0/0"]
  }
}
