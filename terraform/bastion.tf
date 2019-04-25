# EC2 instance to allow DB connections
resource "aws_instance" "bastion" {
  ami                         = "ami-02c6024b3d5467e4a"
  key_name                    = "${aws_key_pair.bastion_key.key_name}"
  instance_type               = "t2.micro"
  vpc_security_group_ids      = ["${aws_security_group.bastion.id}"]
  subnet_id                   = "${aws_subnet.public.0.id}"
  associate_public_ip_address = true
}

resource "aws_key_pair" "bastion_key" {
  key_name   = "id_rsa"
  public_key = "${var.marty_id_rsa_pub}"
}

resource "local_file" "bastion_ip_address" {
  content  = "${aws_instance.bastion.public_ip}"
  filename = "foo.bar"
}

output "bastion_ip_address" {
  value = "${aws_instance.bastion.public_ip}"
}

