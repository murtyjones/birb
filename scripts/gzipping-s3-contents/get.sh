#!/bin/bash

aws s3 sync s3://birb-edgar-filings ./gotten --exclude '*' --include '*.txt'
