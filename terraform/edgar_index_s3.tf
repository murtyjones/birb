resource "aws_s3_bucket" "edgar_indexes" {
  bucket = "edgar-indexes"
  acl    = "private"

  tags = {
    Name = "Edgar Indexes"
  }
}
