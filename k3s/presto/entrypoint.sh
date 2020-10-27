#!/bin/bash

# bash -c "echo -e \"$(cat hive.properties.template)\" > etc/catalog/hive.properties"
bash -c "echo -e \"$(cat etc/node.properties.template)\" > etc/node.properties"
bash -c "echo -e \"$(cat etc/config.properties.template)\" > etc/config.properties"
# bash -c "echo -e \"$(cat etc/alluxio-site.properties.template)\" > etc/alluxio-site.properties"
bin/launcher run
