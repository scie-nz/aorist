import download_data_from_remote_web_location
import download_data_from_remote_pushshift_api_location_to_newline_delimited_json
import convert_json_to_csv
import upload_data_to_minio
from aorist import AoristConstraintProgram, bash_module, sql_module

import os
import re

hive_directories_created = sql_module("recipes/hive_directories_created.presto.sql")
json_table_schemas_created = sql_module("recipes/json_table_schemas_created.presto.sql")
download_data_from_remote_web_location = bash_module("recipes/download_data_from_remote_web_location.sh")

# TODO: change to having multiple programs for each constraint
programs = {
    k.name : [AoristConstraintProgram(v)]
    for k, v in
    list(download_data_from_remote_pushshift_api_location_to_newline_delimited_json.programs.items()) + \
    list(upload_data_to_minio.programs.items()) + \
    list(json_table_schemas_created.programs.items()) + \
    list(convert_json_to_csv.programs.items()) + \
    list(hive_directories_created.programs.items()) + \
    list(download_data_from_remote_web_location.programs.items())
}
