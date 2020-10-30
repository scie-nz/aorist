#!/bin/bash
/usr/bin/entrypoint &
gitea admin user create --username gitadmin --password=gitadmin --email gitadmin@local --admin
export TOKEN=$(curl -X POST --url http://gitadmin:gitadmin@localhost:3000/api/v1/users/gitadmin/tokens -H "accept: application/json" -H "Content-Type: application/json" -d "{ \"name\": \"gitadmin_token\" }" | jq -r '.sha1')
