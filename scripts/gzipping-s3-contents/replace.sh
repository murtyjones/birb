#!/usr/bin/env bash

# Replace all .txt files in S3 with the gzipped files

# YOU WILL NEED TO REMOVE --dryrun AND REPLACE IT WHEN DONE
aws s3 sync ./gotten s3://birb-edgar-filings \
    --exclude '*' --include '*.txt.gz' \
    --content-encoding gzip --content-type text/html --dryrun
