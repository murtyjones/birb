# Outputs to log:
output "alb_hostname" {
  value = "${aws_alb.main.dns_name}"
}

output "bastion_ip_address" {
  value = "${aws_instance.bastion.public_ip}"
}

# Outputs to write to disk:
resource "local_file" "bastion_ip_address" {
  content  = "${aws_instance.bastion.public_ip}"
  filename = "${path.module}/out/bastion_ip_address"
}

resource "local_file" "rds_db_name" {
  content  = "${aws_db_instance.birb.name}"
  filename = "${path.module}/out/rds_db_name"
}

resource "local_file" "rds_db_port" {
  content  = "${aws_db_instance.birb.port}"
  filename = "${path.module}/out/rds_db_port"
}

resource "local_file" "rds_db_address" {
  content  = "${aws_db_instance.birb.address}"
  filename = "${path.module}/out/rds_db_address"
}

resource "local_file" "rds_db_username" {
  content  = "${var.rds_username}"
  filename = "${path.module}/out/rds_db_username"
}

resource "local_file" "rds_db_password" {
  content  = "${var.rds_password}"
  filename = "${path.module}/out/rds_db_password"
}
