#!/bin/bash
bash -c "echo -e \"$(cat /conf/ranger-admin-install.properties.template)\" > \
  /usr/lib/ranger/ranger-2.1.0-admin/install.properties"
mysql -h $MYSQL_HOST -P $MYSQL_PORT -u $MYSQL_USER --password="$MYSQL_PASSWORD" \
  --execute="CREATE DATABASE IF NOT EXISTS $RANGER_DB;"
cd /usr/lib/ranger/ranger-2.1.0-admin/ && ./setup_kube.sh
ranger-admin start
while true; do sleep 30; done;
