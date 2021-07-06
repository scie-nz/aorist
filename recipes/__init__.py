import download_data_from_remote_pushshift_api_location_to_newline_delimited_json
import convert_json_to_csv
from aorist import AoristConstraintProgram, sql_module

import os
import re

file_name = "recipes/hive_directories_created.presto.sql"
hive_directories_created = sql_module(file_name)

# TODO: change to having multiple programs for each constraint
programs = {
    k.name : [AoristConstraintProgram(v)]
    for k, v in
    list(download_data_from_remote_pushshift_api_location_to_newline_delimited_json.programs.items()) + \
    list(convert_json_to_csv.programs.items()) + \
    list(hive_directories_created.programs.items())
}
