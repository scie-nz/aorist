from google.cloud import storage


def download_blob_to_file(bucket_name, blob_name, tmp_dir, file_name):
    client = storage.Client.from_service_account_json("/gcloud/social_norms.json")
    bucket = client.bucket(bucket_name)
    blob = bucket.blob(blob_name)
    dest = "%s/%s" % (tmp_dir, file_name)
    blob.download_to_filename(dest)


params_hive_schemas_created = {
    "ca_HepPh": {
        "schema": hive_schemas_created_schema,
        "data_format": "ORC",
        "table_name": "ca_HepPh",
    },
    "sentinel": {
        "schema": """
    granule_id VARCHAR,
    product_id VARCHAR,
    datatake_identifier VARCHAR,
    mgrs_tile VARCHAR,
    sensing_time BIGINT,
    total_size BIGINT,
    cloud_cover VARCHAR,
    geometric_quality_flag VARCHAR,
    generation_time BIGINT,
    north_lat REAL COMMENT 'Northern latitude of the tile`s bounding box.',
    south_lat REAL COMMENT 'Southern latitude of the tile`s bounding box.',
    west_lon REAL COMMENT 'Western longitude of the tile`s bounding box.',
    east_lon REAL COMMENT 'Eastern longitude of the tile`s bounding box.',
    base_url VARCHAR
""",
        "data_format": "ORC",
        "table_name": "sentinel-2-metadata-table",
    },
    "ca_HepTh": {
        "schema": hive_schemas_created_schema,
        "data_format": "ORC",
        "table_name": "ca_HepTh",
    },
    "web_NotreDame": {
        "schema": hive_schemas_created_schema,
        "data_format": "ORC",
        "table_name": "web_NotreDame",
    },
    "web_Google": {
        "schema": hive_schemas_created_schema,
        "data_format": "ORC",
        "table_name": "web_Google",
    },
    "amazon0601": {
        "schema": hive_schemas_created_schema,
        "data_format": "ORC",
        "table_name": "amazon0601",
    },
    "web_BerkStan": {
        "schema": hive_schemas_created_schema,
        "data_format": "ORC",
        "table_name": "web_BerkStan",
    },
    "web_Stanford": {
        "schema": hive_schemas_created_schema,
        "data_format": "ORC",
        "table_name": "web_Stanford",
    },
    "amazon0312": {
        "schema": hive_schemas_created_schema,
        "data_format": "ORC",
        "table_name": "amazon0312",
    },
    "amazon0302": {
        "schema": hive_schemas_created_schema,
        "data_format": "ORC",
        "table_name": "amazon0302",
    },
    "ca_AstroPh": {
        "schema": hive_schemas_created_schema,
        "data_format": "ORC",
        "table_name": "ca_AstroPh",
    },
    "ca_CondMat": {
        "schema": hive_schemas_created_schema,
        "data_format": "ORC",
        "table_name": "ca_CondMat",
    },
    "amazon0505": {
        "schema": hive_schemas_created_schema,
        "data_format": "ORC",
        "table_name": "amazon0505",
    },
    "ca_GrQc": {
        "schema": hive_schemas_created_schema,
        "data_format": "ORC",
        "table_name": "ca_GrQc",
    },
}
for (t, params) in tasks_hive_schemas_created.items():
    tasks_hive_schemas_created[t] = ShellTask(
        schema=params["schema"],
        data_format=params["data_format"],
        table_name=params["table_name"],
    )
    flow.add_node(tasks_hive_schemas_created[t])
params_replicated_schema = {
    "ca_HepPh": {"dependencies": tasks_hive_schemas_created["ca_HepPh"]},
    "ca_HepTh": {"dependencies": tasks_hive_schemas_created["ca_HepTh"]},
    "web_NotreDame": {"dependencies": tasks_hive_schemas_created["web_NotreDame"]},
    "ca_CondMat": {"dependencies": tasks_hive_schemas_created["ca_CondMat"]},
    "web_BerkStan": {"dependencies": tasks_hive_schemas_created["web_BerkStan"]},
    "ca_GrQc": {"dependencies": tasks_hive_schemas_created["ca_GrQc"]},
    "ca_AstroPh": {"dependencies": tasks_hive_schemas_created["ca_AstroPh"]},
    "web_Stanford": {"dependencies": tasks_hive_schemas_created["web_Stanford"]},
    "amazon0505": {"dependencies": tasks_hive_schemas_created["amazon0505"]},
    "amazon0302": {"dependencies": tasks_hive_schemas_created["amazon0302"]},
    "amazon0601": {"dependencies": tasks_hive_schemas_created["amazon0601"]},
    "sentinel": {"dependencies": tasks_hive_schemas_created["sentinel"]},
    "web_Google": {"dependencies": tasks_hive_schemas_created["web_Google"]},
    "amazon0312": {"dependencies": tasks_hive_schemas_created["amazon0312"]},
}
for (t, params) in tasks_replicated_schema.items():
    tasks_replicated_schema[t] = ConstantTask()
    flow.add_node(tasks_replicated_schema[t])
    flow.add_edge(tasks_replicated_schema[t], params["dependencies"])
download_data_from_remote_gcs_location__sentinel_location = download_blob_to_file(
    "gcp-public-data-sentinel2",
    "index.csv.gz-backup",
    "/tmp/sentinel2",
    "sentinel-2-metadata-table",
)
flow.add_node(download_data_from_remote_gcs_location__sentinel_location)
params_download_data_from_remote_web_location = {
    "web_Google": {
        "tmp_dir": "/tmp/snap/web_Google/",
        "address": "https://snap.stanford.edu/data/web-Google.txt.gz",
        "file_name": "web_Google",
    },
    "web_NotreDame": {
        "tmp_dir": "/tmp/snap/web_NotreDame/",
        "address": "https://snap.stanford.edu/data/web-NotreDame.txt.gz",
        "file_name": "web_NotreDame",
    },
    "web_Stanford": {
        "tmp_dir": "/tmp/snap/web_Stanford/",
        "address": "https://snap.stanford.edu/data/web-Stanford.txt.gz",
        "file_name": "web_Stanford",
    },
    "ca_CondMat": {
        "tmp_dir": "/tmp/snap/ca_CondMat/",
        "address": "https://snap.stanford.edu/data/ca-CondMat.txt.gz",
        "file_name": "ca_CondMat",
    },
    "amazon0302": {
        "tmp_dir": "/tmp/snap/amazon0302/",
        "address": "https://snap.stanford.edu/data/amazon0302.txt.gz",
        "file_name": "amazon0302",
    },
    "ca_AstroPh": {
        "tmp_dir": "/tmp/snap/ca_AstroPh/",
        "address": "https://snap.stanford.edu/data/ca-AstroPh.txt.gz",
        "file_name": "ca_AstroPh",
    },
    "amazon0312": {
        "tmp_dir": "/tmp/snap/amazon0312/",
        "address": "https://snap.stanford.edu/data/amazon0312.txt.gz",
        "file_name": "amazon0312",
    },
    "amazon0601": {
        "tmp_dir": "/tmp/snap/amazon0601/",
        "address": "https://snap.stanford.edu/data/amazon0601.txt.gz",
        "file_name": "amazon0601",
    },
    "ca_GrQc": {
        "tmp_dir": "/tmp/snap/ca_GrQc/",
        "address": "https://snap.stanford.edu/data/ca-GrQc.txt.gz",
        "file_name": "ca_GrQc",
    },
    "web_BerkStan": {
        "tmp_dir": "/tmp/snap/web_BerkStan/",
        "address": "https://snap.stanford.edu/data/web-BerkStan.txt.gz",
        "file_name": "web_BerkStan",
    },
    "ca_HepTh": {
        "tmp_dir": "/tmp/snap/ca_HepTh/",
        "address": "https://snap.stanford.edu/data/ca-HepTh.txt.gz",
        "file_name": "ca_HepTh",
    },
    "amazon0505": {
        "tmp_dir": "/tmp/snap/amazon0505/",
        "address": "https://snap.stanford.edu/data/amazon0505.txt.gz",
        "file_name": "amazon0505",
    },
    "ca_HepPh": {
        "tmp_dir": "/tmp/snap/ca_HepPh/",
        "address": "https://snap.stanford.edu/data/ca-HepPh.txt.gz",
        "file_name": "ca_HepPh",
    },
}
for (t, params) in tasks_download_data_from_remote_web_location.items():
    tasks_download_data_from_remote_web_location[t] = ShellTask(
        tmp_dir=params["tmp_dir"],
        address=params["address"],
        file_name=params["file_name"],
    )
    flow.add_node(tasks_download_data_from_remote_web_location[t])
params_decompress = {
    "amazon0312": {
        "dependencies": tasks_download_data_from_remote_web_location["amazon0312"],
        "tmp_dir": "/tmp/snap/amazon0312/",
        "file_name": "amazon0312",
    },
    "web_BerkStan": {
        "dependencies": tasks_download_data_from_remote_web_location["web_BerkStan"],
        "tmp_dir": "/tmp/snap/web_BerkStan/",
        "file_name": "web_BerkStan",
    },
    "web_NotreDame": {
        "dependencies": tasks_download_data_from_remote_web_location["web_NotreDame"],
        "tmp_dir": "/tmp/snap/web_NotreDame/",
        "file_name": "web_NotreDame",
    },
    "web_Stanford": {
        "dependencies": tasks_download_data_from_remote_web_location["web_Stanford"],
        "tmp_dir": "/tmp/snap/web_Stanford/",
        "file_name": "web_Stanford",
    },
    "amazon0601": {
        "dependencies": tasks_download_data_from_remote_web_location["amazon0601"],
        "tmp_dir": "/tmp/snap/amazon0601/",
        "file_name": "amazon0601",
    },
    "ca_GrQc": {
        "dependencies": tasks_download_data_from_remote_web_location["ca_GrQc"],
        "tmp_dir": "/tmp/snap/ca_GrQc/",
        "file_name": "ca_GrQc",
    },
    "ca_AstroPh": {
        "dependencies": tasks_download_data_from_remote_web_location["ca_AstroPh"],
        "tmp_dir": "/tmp/snap/ca_AstroPh/",
        "file_name": "ca_AstroPh",
    },
    "amazon0302": {
        "dependencies": tasks_download_data_from_remote_web_location["amazon0302"],
        "tmp_dir": "/tmp/snap/amazon0302/",
        "file_name": "amazon0302",
    },
    "ca_HepPh": {
        "dependencies": tasks_download_data_from_remote_web_location["ca_HepPh"],
        "tmp_dir": "/tmp/snap/ca_HepPh/",
        "file_name": "ca_HepPh",
    },
    "sentinel": {
        "dependencies": download_data_from_remote_gcs_location__sentinel_location,
        "tmp_dir": "/tmp/sentinel2",
        "file_name": "sentinel-2-metadata-table",
    },
    "amazon0505": {
        "dependencies": tasks_download_data_from_remote_web_location["amazon0505"],
        "tmp_dir": "/tmp/snap/amazon0505/",
        "file_name": "amazon0505",
    },
    "ca_CondMat": {
        "dependencies": tasks_download_data_from_remote_web_location["ca_CondMat"],
        "tmp_dir": "/tmp/snap/ca_CondMat/",
        "file_name": "ca_CondMat",
    },
    "web_Google": {
        "dependencies": tasks_download_data_from_remote_web_location["web_Google"],
        "tmp_dir": "/tmp/snap/web_Google/",
        "file_name": "web_Google",
    },
    "ca_HepTh": {
        "dependencies": tasks_download_data_from_remote_web_location["ca_HepTh"],
        "tmp_dir": "/tmp/snap/ca_HepTh/",
        "file_name": "ca_HepTh",
    },
}
for (t, params) in tasks_decompress.items():
    tasks_decompress[t] = ShellTask(
        tmp_dir=params["tmp_dir"], file_name=params["file_name"]
    )
    flow.add_node(tasks_decompress[t])
    flow.add_edge(tasks_decompress[t], params["dependencies"])
params_remove_file_header = {
    "ca_GrQc": {
        "dependencies": tasks_decompress["ca_GrQc"],
        "n": "2",
        "file_name": "ca_GrQc",
        "tmp_dir": "/tmp/snap/ca_GrQc/",
    },
    "ca_CondMat": {
        "dependencies": tasks_decompress["ca_CondMat"],
        "n": "2",
        "file_name": "ca_CondMat",
        "tmp_dir": "/tmp/snap/ca_CondMat/",
    },
    "web_Google": {
        "dependencies": tasks_decompress["web_Google"],
        "n": "2",
        "file_name": "web_Google",
        "tmp_dir": "/tmp/snap/web_Google/",
    },
    "amazon0302": {
        "dependencies": tasks_decompress["amazon0302"],
        "n": "2",
        "file_name": "amazon0302",
        "tmp_dir": "/tmp/snap/amazon0302/",
    },
    "web_Stanford": {
        "dependencies": tasks_decompress["web_Stanford"],
        "n": "2",
        "file_name": "web_Stanford",
        "tmp_dir": "/tmp/snap/web_Stanford/",
    },
    "ca_HepPh": {
        "dependencies": tasks_decompress["ca_HepPh"],
        "n": "2",
        "file_name": "ca_HepPh",
        "tmp_dir": "/tmp/snap/ca_HepPh/",
    },
    "web_BerkStan": {
        "dependencies": tasks_decompress["web_BerkStan"],
        "n": "2",
        "file_name": "web_BerkStan",
        "tmp_dir": "/tmp/snap/web_BerkStan/",
    },
    "web_NotreDame": {
        "dependencies": tasks_decompress["web_NotreDame"],
        "n": "2",
        "file_name": "web_NotreDame",
        "tmp_dir": "/tmp/snap/web_NotreDame/",
    },
    "amazon0312": {
        "dependencies": tasks_decompress["amazon0312"],
        "n": "2",
        "file_name": "amazon0312",
        "tmp_dir": "/tmp/snap/amazon0312/",
    },
    "ca_AstroPh": {
        "dependencies": tasks_decompress["ca_AstroPh"],
        "n": "2",
        "file_name": "ca_AstroPh",
        "tmp_dir": "/tmp/snap/ca_AstroPh/",
    },
    "sentinel": {
        "dependencies": tasks_decompress["sentinel"],
        "n": "2",
        "file_name": "sentinel-2-metadata-table",
        "tmp_dir": "/tmp/sentinel2",
    },
    "ca_HepTh": {
        "dependencies": tasks_decompress["ca_HepTh"],
        "n": "2",
        "file_name": "ca_HepTh",
        "tmp_dir": "/tmp/snap/ca_HepTh/",
    },
    "amazon0601": {
        "dependencies": tasks_decompress["amazon0601"],
        "n": "2",
        "file_name": "amazon0601",
        "tmp_dir": "/tmp/snap/amazon0601/",
    },
    "amazon0505": {
        "dependencies": tasks_decompress["amazon0505"],
        "n": "2",
        "file_name": "amazon0505",
        "tmp_dir": "/tmp/snap/amazon0505/",
    },
}
for (t, params) in tasks_remove_file_header.items():
    tasks_remove_file_header[t] = ShellTask(
        n=params["n"], file_name=params["file_name"], tmp_dir=params["tmp_dir"]
    )
    flow.add_node(tasks_remove_file_header[t])
    flow.add_edge(tasks_remove_file_header[t], params["dependencies"])
params_file_ready_for_upload = {
    "ca_HepTh": {"dependencies": tasks_remove_file_header["ca_HepTh"]},
    "ca_CondMat": {"dependencies": tasks_remove_file_header["ca_CondMat"]},
    "web_Google": {"dependencies": tasks_remove_file_header["web_Google"]},
    "web_NotreDame": {"dependencies": tasks_remove_file_header["web_NotreDame"]},
    "ca_GrQc": {"dependencies": tasks_remove_file_header["ca_GrQc"]},
    "amazon0312": {"dependencies": tasks_remove_file_header["amazon0312"]},
    "amazon0505": {"dependencies": tasks_remove_file_header["amazon0505"]},
    "amazon0302": {"dependencies": tasks_remove_file_header["amazon0302"]},
    "amazon0601": {"dependencies": tasks_remove_file_header["amazon0601"]},
    "web_Stanford": {"dependencies": tasks_remove_file_header["web_Stanford"]},
    "ca_AstroPh": {"dependencies": tasks_remove_file_header["ca_AstroPh"]},
    "web_BerkStan": {"dependencies": tasks_remove_file_header["web_BerkStan"]},
    "ca_HepPh": {"dependencies": tasks_remove_file_header["ca_HepPh"]},
}
for (t, params) in tasks_file_ready_for_upload.items():
    tasks_file_ready_for_upload[t] = ConstantTask()
    flow.add_node(tasks_file_ready_for_upload[t])
    flow.add_edge(tasks_file_ready_for_upload[t], params["dependencies"])
params_convert_csv_to_orc = {
    "amazon0601": {
        "dependencies": tasks_remove_file_header["amazon0601"],
        "tmp_dir": "/tmp/snap/amazon0601/",
        "table_name": "amazon0601",
        "schema": "from_id:BIGINT,to_id:BIGINT",
    },
    "web_NotreDame": {
        "dependencies": tasks_remove_file_header["web_NotreDame"],
        "tmp_dir": "/tmp/snap/web_NotreDame/",
        "table_name": "web_NotreDame",
        "schema": "from_id:BIGINT,to_id:BIGINT",
    },
    "ca_HepPh": {
        "dependencies": tasks_remove_file_header["ca_HepPh"],
        "tmp_dir": "/tmp/snap/ca_HepPh/",
        "table_name": "ca_HepPh",
        "schema": "from_id:BIGINT,to_id:BIGINT",
    },
    "amazon0505": {
        "dependencies": tasks_remove_file_header["amazon0505"],
        "tmp_dir": "/tmp/snap/amazon0505/",
        "table_name": "amazon0505",
        "schema": "from_id:BIGINT,to_id:BIGINT",
    },
    "sentinel": {
        "dependencies": tasks_remove_file_header["sentinel"],
        "tmp_dir": "/tmp/sentinel2",
        "table_name": "sentinel-2-metadata-table",
        "schema": (
            "granule_id:STRING,product_id:STRING"
            ",datatake_identifier:STRING,mgrs_tile:STRING"
            ",sensing_time:BIGINT,total_size:BIGINT,cloud_cover:STRING"
            ",geometric_quality_flag:STRING,generation_time:BIGINT"
            ",north_lat:FLOAT,south_lat:FLOAT,west_lon:FLOAT"
            ",east_lon:FLOAT,base_url:STRING"
        ),
    },
    "ca_HepTh": {
        "dependencies": tasks_remove_file_header["ca_HepTh"],
        "tmp_dir": "/tmp/snap/ca_HepTh/",
        "table_name": "ca_HepTh",
        "schema": "from_id:BIGINT,to_id:BIGINT",
    },
    "ca_CondMat": {
        "dependencies": tasks_remove_file_header["ca_CondMat"],
        "tmp_dir": "/tmp/snap/ca_CondMat/",
        "table_name": "ca_CondMat",
        "schema": "from_id:BIGINT,to_id:BIGINT",
    },
    "amazon0312": {
        "dependencies": tasks_remove_file_header["amazon0312"],
        "tmp_dir": "/tmp/snap/amazon0312/",
        "table_name": "amazon0312",
        "schema": "from_id:BIGINT,to_id:BIGINT",
    },
    "ca_AstroPh": {
        "dependencies": tasks_remove_file_header["ca_AstroPh"],
        "tmp_dir": "/tmp/snap/ca_AstroPh/",
        "table_name": "ca_AstroPh",
        "schema": "from_id:BIGINT,to_id:BIGINT",
    },
    "amazon0302": {
        "dependencies": tasks_remove_file_header["amazon0302"],
        "tmp_dir": "/tmp/snap/amazon0302/",
        "table_name": "amazon0302",
        "schema": "from_id:BIGINT,to_id:BIGINT",
    },
    "web_Google": {
        "dependencies": tasks_remove_file_header["web_Google"],
        "tmp_dir": "/tmp/snap/web_Google/",
        "table_name": "web_Google",
        "schema": "from_id:BIGINT,to_id:BIGINT",
    },
    "web_Stanford": {
        "dependencies": tasks_remove_file_header["web_Stanford"],
        "tmp_dir": "/tmp/snap/web_Stanford/",
        "table_name": "web_Stanford",
        "schema": "from_id:BIGINT,to_id:BIGINT",
    },
    "web_BerkStan": {
        "dependencies": tasks_remove_file_header["web_BerkStan"],
        "tmp_dir": "/tmp/snap/web_BerkStan/",
        "table_name": "web_BerkStan",
        "schema": "from_id:BIGINT,to_id:BIGINT",
    },
    "ca_GrQc": {
        "dependencies": tasks_remove_file_header["ca_GrQc"],
        "tmp_dir": "/tmp/snap/ca_GrQc/",
        "table_name": "ca_GrQc",
        "schema": "from_id:BIGINT,to_id:BIGINT",
    },
}
for (t, params) in tasks_convert_csv_to_orc.items():
    tasks_convert_csv_to_orc[t] = ShellTask(
        tmp_dir=params["tmp_dir"],
        table_name=params["table_name"],
        schema=params["schema"],
    )
    flow.add_node(tasks_convert_csv_to_orc[t])
    flow.add_edge(tasks_convert_csv_to_orc[t], params["dependencies"])
params_upload_data_to_local = {
    "amazon0601": {"dependencies": tasks_convert_csv_to_orc["amazon0601"]},
    "amazon0302": {"dependencies": tasks_convert_csv_to_orc["amazon0302"]},
    "ca_HepPh": {"dependencies": tasks_convert_csv_to_orc["ca_HepPh"]},
    "web_BerkStan": {"dependencies": tasks_convert_csv_to_orc["web_BerkStan"]},
    "web_Google": {"dependencies": tasks_convert_csv_to_orc["web_Google"]},
    "web_Stanford": {"dependencies": tasks_convert_csv_to_orc["web_Stanford"]},
    "ca_HepTh": {"dependencies": tasks_convert_csv_to_orc["ca_HepTh"]},
    "ca_AstroPh": {"dependencies": tasks_convert_csv_to_orc["ca_AstroPh"]},
    "amazon0312": {"dependencies": tasks_convert_csv_to_orc["amazon0312"]},
    "amazon0505": {"dependencies": tasks_convert_csv_to_orc["amazon0505"]},
    "ca_GrQc": {"dependencies": tasks_convert_csv_to_orc["ca_GrQc"]},
    "web_NotreDame": {"dependencies": tasks_convert_csv_to_orc["web_NotreDame"]},
    "ca_CondMat": {"dependencies": tasks_convert_csv_to_orc["ca_CondMat"]},
}
for (t, params) in tasks_upload_data_to_local.items():
    tasks_upload_data_to_local[t] = ConstantTask()
    flow.add_node(tasks_upload_data_to_local[t])
    flow.add_edge(tasks_upload_data_to_local[t], params["dependencies"])
replicated_data__sentinel = ConstantTask()
flow.add_node(replicated_data__sentinel)
for dep in [tasks_remove_file_header["sentinel"], tasks_convert_csv_to_orc["sentinel"]]:
    flow.add_edge(replicated_data__sentinel, dep)
params_replicated_assets = {
    "web_NotreDame": {
        "dependencies": [
            tasks_replicated_schema["web_NotreDame"],
            tasks_file_ready_for_upload["web_NotreDame"],
            tasks_upload_data_to_local["web_NotreDame"],
        ]
    },
    "ca_HepPh": {
        "dependencies": [
            tasks_replicated_schema["ca_HepPh"],
            tasks_file_ready_for_upload["ca_HepPh"],
            tasks_upload_data_to_local["ca_HepPh"],
        ]
    },
    "web_Google": {
        "dependencies": [
            tasks_replicated_schema["web_Google"],
            tasks_file_ready_for_upload["web_Google"],
            tasks_upload_data_to_local["web_Google"],
        ]
    },
    "amazon0505": {
        "dependencies": [
            tasks_replicated_schema["amazon0505"],
            tasks_file_ready_for_upload["amazon0505"],
            tasks_upload_data_to_local["amazon0505"],
        ]
    },
    "amazon0312": {
        "dependencies": [
            tasks_replicated_schema["amazon0312"],
            tasks_file_ready_for_upload["amazon0312"],
            tasks_upload_data_to_local["amazon0312"],
        ]
    },
    "ca_GrQc": {
        "dependencies": [
            tasks_replicated_schema["ca_GrQc"],
            tasks_file_ready_for_upload["ca_GrQc"],
            tasks_upload_data_to_local["ca_GrQc"],
        ]
    },
    "amazon0302": {
        "dependencies": [
            tasks_replicated_schema["amazon0302"],
            tasks_file_ready_for_upload["amazon0302"],
            tasks_upload_data_to_local["amazon0302"],
        ]
    },
    "amazon0601": {
        "dependencies": [
            tasks_replicated_schema["amazon0601"],
            tasks_file_ready_for_upload["amazon0601"],
            tasks_upload_data_to_local["amazon0601"],
        ]
    },
    "ca_AstroPh": {
        "dependencies": [
            tasks_replicated_schema["ca_AstroPh"],
            tasks_file_ready_for_upload["ca_AstroPh"],
            tasks_upload_data_to_local["ca_AstroPh"],
        ]
    },
    "web_Stanford": {
        "dependencies": [
            tasks_replicated_schema["web_Stanford"],
            tasks_file_ready_for_upload["web_Stanford"],
            tasks_upload_data_to_local["web_Stanford"],
        ]
    },
    "web_BerkStan": {
        "dependencies": [
            tasks_replicated_schema["web_BerkStan"],
            tasks_file_ready_for_upload["web_BerkStan"],
            tasks_upload_data_to_local["web_BerkStan"],
        ]
    },
    "ca_CondMat": {
        "dependencies": [
            tasks_replicated_schema["ca_CondMat"],
            tasks_file_ready_for_upload["ca_CondMat"],
            tasks_upload_data_to_local["ca_CondMat"],
        ]
    },
    "ca_HepTh": {
        "dependencies": [
            tasks_replicated_schema["ca_HepTh"],
            tasks_file_ready_for_upload["ca_HepTh"],
            tasks_upload_data_to_local["ca_HepTh"],
        ]
    },
}
for (t, params) in tasks_replicated_assets.items():
    tasks_replicated_assets[t] = ConstantTask()
    flow.add_node(tasks_replicated_assets[t])
    flow.add_edge(tasks_replicated_assets[t], params["dependencies"])
replicated_data_sets__sentinel = ConstantTask()
flow.add_node(replicated_data_sets__sentinel)
for dep in [tasks_replicated_schema["sentinel"], replicated_data__sentinel]:
    flow.add_edge(replicated_data_sets__sentinel, dep)
replicated_data_sets__snap = ConstantTask()
flow.add_node(replicated_data_sets__snap)
for dep in [
    tasks_replicated_assets["web_NotreDame"],
    tasks_replicated_assets["ca_HepPh"],
    tasks_replicated_assets["web_Google"],
    tasks_replicated_assets["amazon0505"],
    tasks_replicated_assets["amazon0312"],
    tasks_replicated_assets["ca_GrQc"],
    tasks_replicated_assets["amazon0302"],
    tasks_replicated_assets["amazon0601"],
    tasks_replicated_assets["ca_AstroPh"],
    tasks_replicated_assets["web_Stanford"],
    tasks_replicated_assets["web_BerkStan"],
    tasks_replicated_assets["ca_CondMat"],
    tasks_replicated_assets["ca_HepTh"],
]:
    flow.add_edge(replicated_data_sets__snap, dep)
is_consistent__basic_data_setup = ConstantTask()
flow.add_node(is_consistent__basic_data_setup)
for dep in [replicated_data_sets__sentinel, replicated_data_sets__snap]:
    flow.add_edge(is_consistent__basic_data_setup, dep)
params_is_audited_table = {
    "amazon0505": {
        "dependencies": [replicated_data_sets__sentinel, replicated_data_sets__snap]
    },
    "web_BerkStan": {
        "dependencies": [replicated_data_sets__sentinel, replicated_data_sets__snap]
    },
    "ca_HepTh": {
        "dependencies": [replicated_data_sets__sentinel, replicated_data_sets__snap]
    },
    "amazon0601": {
        "dependencies": [replicated_data_sets__sentinel, replicated_data_sets__snap]
    },
    "amazon0302": {
        "dependencies": [replicated_data_sets__sentinel, replicated_data_sets__snap]
    },
    "web_NotreDame": {
        "dependencies": [replicated_data_sets__sentinel, replicated_data_sets__snap]
    },
    "ca_HepPh": {
        "dependencies": [replicated_data_sets__sentinel, replicated_data_sets__snap]
    },
    "sentinel": {
        "dependencies": [replicated_data_sets__sentinel, replicated_data_sets__snap]
    },
    "ca_CondMat": {
        "dependencies": [replicated_data_sets__sentinel, replicated_data_sets__snap]
    },
    "web_Google": {
        "dependencies": [replicated_data_sets__sentinel, replicated_data_sets__snap]
    },
    "ca_GrQc": {
        "dependencies": [replicated_data_sets__sentinel, replicated_data_sets__snap]
    },
    "web_Stanford": {
        "dependencies": [replicated_data_sets__sentinel, replicated_data_sets__snap]
    },
    "ca_AstroPh": {
        "dependencies": [replicated_data_sets__sentinel, replicated_data_sets__snap]
    },
    "amazon0312": {
        "dependencies": [replicated_data_sets__sentinel, replicated_data_sets__snap]
    },
}
for (t, params) in tasks_is_audited_table.items():
    tasks_is_audited_table[t] = ConstantTask()
    flow.add_node(tasks_is_audited_table[t])
    flow.add_edge(tasks_is_audited_table[t], params["dependencies"])
is_audited__basic_data_setup = ConstantTask()
flow.add_node(is_audited__basic_data_setup)
for dep in [
    tasks_is_audited_table["amazon0505"],
    tasks_is_audited_table["web_BerkStan"],
    tasks_is_audited_table["ca_HepTh"],
    tasks_is_audited_table["amazon0601"],
    tasks_is_audited_table["amazon0302"],
    tasks_is_audited_table["web_NotreDame"],
    tasks_is_audited_table["ca_HepPh"],
    tasks_is_audited_table["sentinel"],
    tasks_is_audited_table["ca_CondMat"],
    tasks_is_audited_table["web_Google"],
    tasks_is_audited_table["ca_GrQc"],
    tasks_is_audited_table["web_Stanford"],
    tasks_is_audited_table["ca_AstroPh"],
    tasks_is_audited_table["amazon0312"],
]:
    flow.add_edge(is_audited__basic_data_setup, dep)
