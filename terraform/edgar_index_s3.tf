resource "aws_s3_bucket" "birb_edgar_indexes" {
  bucket = "birb-edgar-indexes"
  acl    = "private"

  tags = {
    Name = "Edgar Indexes"
  }
}
