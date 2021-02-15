#!/bin/bash
mkdir -p conf
bash -c "echo -e \"$(cat metastore-site.xml.template)\" > conf/metastore-site.xml"
mysql -h $MYSQL_HOST -P $MYSQL_PORT -u $MYSQL_USER --password="$MYSQL_PASSWORD" \
  --execute="CREATE DATABASE IF NOT EXISTS $METASTORE_DB;"
bin/schematool -initSchema -dbType mysql -ifNotExists
bin/start-metastore
