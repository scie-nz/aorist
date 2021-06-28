import download_data_from_remote_pushshift_api_location_to_newline_delimited_json
from aorist import AoristConstraintProgram
# TODO: change to having multiple programs for each constraint
programs = {
    k.name : [AoristConstraintProgram(v)] 
    for k, v in 
    download_data_from_remote_pushshift_api_location_to_newline_delimited_json.programs.items()
}
