resource "aws_db_subnet_group" "birb" {
  name       = "birb"
  subnet_ids = ["${aws_subnet.private.*.id}"]
}

resource "aws_db_instance" "birb" {
  name                   = "${var.rds_db_name}"
  identifier             = "birb"
  username               = "${var.rds_username}"
  password               = "${var.rds_password}"
  port                   = "5432"
  engine                 = "postgres"
  engine_version         = "11.1"
  instance_class         = "${var.rds_instance}"
  allocated_storage      = "10"
  storage_encrypted      = false
  vpc_security_group_ids = ["${aws_security_group.birb_rds.id}"]
  db_subnet_group_name   = "${aws_db_subnet_group.birb.name}"
  multi_az               = "${var.multi_az}"
  storage_type           = "gp2"
  publicly_accessible    = false
  deletion_protection    = true

  # snapshot_identifier       = "birb"
  allow_major_version_upgrade = false
  auto_minor_version_upgrade  = false
  apply_immediately           = true
  maintenance_window          = "sun:02:00-sun:04:00"
  skip_final_snapshot         = true
  copy_tags_to_snapshot       = true
  backup_retention_period     = 7
  backup_window               = "04:00-06:00"
  final_snapshot_identifier   = "birb"
}
