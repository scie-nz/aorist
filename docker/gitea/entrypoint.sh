#!/bin/bash
(/usr/bin/entrypoint) &
status=$?
if [ $status -ne 0 ]; then
  echo "Failed to start gitea admin: $status"
  exit $status
fi
gitea admin user create --username gitadmin --password=eagerLamprey --email gitadmin@local --admin
sleep 30
export TOKEN=$(curl -X POST --url http://gitadmin:eagerLamprey@localhost:3000/api/v1/users/gitadmin/tokens -H "accept: application/json" -H "Content-Type: application/json" -d "{ \"name\": \"$(date +%s)\" }" | jq -r '.sha1')
echo "$TOKEN" > /admin-token/token

while sleep 60; do
  ps aux |grep s6 |grep -q -v grep
  PROCESS_1_STATUS=$?
  # If the greps above find anything, they exit with 0 status
  # If they are not both 0, then something is wrong
  if [ $PROCESS_1_STATUS -ne 0 ]; then
    echo "Gitea admin crashed."
    exit 1
  fi
done
