locals {
  instance-userdata = <<EOF
#!/bin/bash
echo "${var.marty_id_rsa_laptop_1_pub}" > /home/ec2-user/.ssh/authorized_keys
echo "${var.marty_id_rsa_laptop_2_pub}" >> /home/ec2-user/.ssh/authorized_keys
chown ec2-user.ec2-user /home/ec2-user/.ssh/authorized_keys
chmod 400 /home/ec2-user/.ssh/authorized_keys
EOF
}

# EC2 instance to allow DB connections
resource "aws_instance" "bastion" {
  ami                         = "ami-02c6024b3d5467e4a"
  instance_type               = "t3a.nano"
  vpc_security_group_ids      = [aws_security_group.bastion.id]
  subnet_id                   = aws_subnet.public[0].id
  associate_public_ip_address = true
  user_data_base64            = base64encode(local.instance-userdata)

  tags = {
    Name = "birb-bastion"
  }
}


