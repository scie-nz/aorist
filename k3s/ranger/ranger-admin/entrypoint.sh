#!/bin/bash
bash -c "echo -e \"$(cat /conf/ranger-admin-install.properties.template)\" > \
  /usr/lib/ranger/ranger-2.1.0-admin/install.properties"
cd /usr/lib/ranger/ranger-2.1.0-admin/ranger-admin && setup.sh
while true; do sleep 30; done;
