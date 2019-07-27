resource "aws_s3_bucket" "birb_edgar_filings" {
  bucket = "birb-edgar-filings"
  acl    = "private"

  tags = {
    Name = "Edgar Filings"
  }
}

