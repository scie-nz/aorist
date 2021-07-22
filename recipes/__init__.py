import download_data_from_remote_web_location
import download_data_from_remote_pushshift_api_location_to_newline_delimited_json
import convert_json_to_csv
import fasttext_training_data
import train_fasttext_model
import upload_data_to_minio
from aorist import register_recipes

programs = register_recipes(
    py_modules=[
        download_data_from_remote_pushshift_api_location_to_newline_delimited_json,
        upload_data_to_minio,
        convert_json_to_csv,
        fasttext_training_data,
        train_fasttext_model,
    ],
    sql_modules=[
        "recipes/hive_directories_created.presto.sql",
        "recipes/json_table_schemas_created.presto.sql",
        "recipes/convert_json_table_to_orc_table.presto.sql",
        "recipes/orc_table_schemas_created.presto.sql",
    ],
    bash_modules=[
        "recipes/download_data_from_remote_web_location.sh",
    ],
    r_modules=[
        "recipes/download_data_from_remote_web_location.R",
    ],
)

