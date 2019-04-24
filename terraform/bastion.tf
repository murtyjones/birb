# EC2 instance to allow DB connections
resource "aws_instance" "bastion" {
  ami                         = "ami-969ab1f6"
  key_name                    = "${aws_key_pair.bastion_key.key_name}"
  instance_type               = "t2.micro"
  security_groups             = ["${aws_security_group.bastion.name}"]
  associate_public_ip_address = true
}

resource "aws_key_pair" "bastion_key" {
  key_name   = "id_rsa"
  public_key = "${var.marty_id_rsa_pub}"
}