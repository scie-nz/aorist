from . import download_data_from_remote_web_location
from . import download_data_from_remote_pushshift_api_location_to_newline_delimited_json
from . import convert_json_to_csv
from . import fasttext_training_data
from . import train_fasttext_model
from . import upload_data_to_minio
from . import upload_fasttext_to_minio
import pathlib
from aorist import register_recipes

path = pathlib.Path(__file__).parent.resolve()

programs = register_recipes(
    py_modules=[
        download_data_from_remote_pushshift_api_location_to_newline_delimited_json,
        upload_data_to_minio,
        convert_json_to_csv,
        fasttext_training_data,
        train_fasttext_model,
        upload_fasttext_to_minio,
    ],
    sql_modules=[
        "%s/hive_directories_created.presto.sql" % path,
        "%s/json_table_schemas_created.presto.sql" % path,
        "%s/convert_json_table_to_orc_table.presto.sql" % path,
        "%s/orc_table_schemas_created.presto.sql" % path,
    ],
    bash_modules=[
        "%s/download_data_from_remote_web_location.sh" % path,
    ],
    r_modules=[
        "%s/download_data_from_remote_web_location.R" % path,
    ],
)

