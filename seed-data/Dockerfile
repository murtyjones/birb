FROM mongo

COPY development/companiesArray.json /companiesArray.json

CMD [ "mongoimport", \
      "--host", "mongo_datastore", \
      "--db", "playground", \
      "--collection", "filer", \
      "--type", "json", \
      "--file", "/companiesArray.json", \
      "--jsonArray" \
]
