#!/bin/bash

# Exit immediately if any step errors:
set -e

# Current year and quarter e.g. 2019Q2
current_year_quarter=$(date +%Y)QTR$(( ($(date +%-m)-1)/3+1 ))

BASE_EDGAR_INDEXES_FOLDER=data/edgar-indexes

# Upsert edgar indexes folder
mkdir -p ${BASE_EDGAR_INDEXES_FOLDER}

# Function to get the index for a quarter if it isn't
# already downloaded to our indexes folder
download_index () {
    FILENAME="master.idx"
    FILEPATH=$1"/"$FILENAME
    if test -f "$FILEPATH"; then
        echo "$FILEPATH exists, not downloading."
    else
        echo "$FILEPATH doesn't exist, downloading."
        curl "https://www.sec.gov/Archives/edgar/full-index/$2/$3/$FILENAME" -o ${FILEPATH}
        # wget  -P $1 "https://www.sec.gov/Archives/edgar/full-index/$2/$3/$FILE"
    fi
}

# Get the filings for each year
for year in 2016 2017 2018 2019
do
    past_current_quarter=false
    # Get the filings for each quarter
   for quarter_number in 1 2 3 4
   do
       each_quarter="QTR"${quarter_number}
       each_year_quarter=${year}${each_quarter}
       folder_name=${BASE_EDGAR_INDEXES_FOLDER}/${year}/${each_quarter}
        echo "${each_year_quarter} == ${current_year_quarter}"
       if [[ ${each_year_quarter} == ${current_year_quarter} ]]; then
          past_current_quarter=true
          # If the quarter is the current quarter, we should re-download
          # the current quarter's indexes from edgar.
          echo "Deleting/recreating folder for $each_year_quarter"
          rm -rf ${folder_name}
          mkdir ${folder_name}

       else
        # Otherwise upsert the folder
        echo "Creating folder for $each_year_quarter"
        mkdir -p ${folder_name}
       fi

       download_index ${folder_name} ${year} ${each_quarter}

       # No more quarters in this year, break
       if [[ ${past_current_quarter} == true ]]; then
           break
       fi
   done
done
