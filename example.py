New name: hive_schemas_created__wikitalk
New name: hive_schemas_created__wikitalk
New name: replicated_schema
New name: decompress
New name: convert_csv_to_orc
New name: download_data_from_remote_gcs_location
New name: replicated_schema
New name: remove_file_header
New name: is_consistent
New name: is_audited_table
New name: is_audited
New name: remove_file_header
New name: convert_csv_to_orc
New name: replicated_data_sets
New name: replicated_data_sets
New name: download_data_from_remote
New name: download_data_from_remote
New name: hive_schemas_created__sentinel
New name: hive_schemas_created__sentinel
New name: replicated_data
New name: decompress
New name: hive_schemas_created
New name: hive_schemas_created__wikitalk
New name: replicated_schema
New name: decompress
New name: convert_csv_to_orc
New name: download_data_from_remote_gcs_location
New name: replicated_schema
New name: remove_file_header
New name: is_consistent
New name: is_audited_table
New name: is_audited
New name: remove_file_header
New name: convert_csv_to_orc
New name: replicated_data_sets
New name: replicated_data_sets
New name: download_data_from_remote
New name: download_data_from_remote
New name: hive_schemas_created
New name: hive_schemas_created__sentinel
New name: replicated_data
New name: decompress
hive_schemas_created__wikitalk = ShellTask(command="""
presto -e 'CREATE TABLE IF NOT EXISTS WikiTalk (
    to_id BIGINT,
    from_id BIGINT
) WITH (format=\'ORC\');
'""")
flow.add_node(hive_schemas_created__wikitalk)
download_data_from_remote__wikitalk = ShellTask(command="""
presto -e 'CREATE TABLE IF NOT EXISTS sentinel-2-metadata-table (
    geometric_quality_flag VARCHAR,
    north_lat REAL COMMENT \'Northern latitude of the tile`s bounding box.\',
    mgrs_tile VARCHAR,
    sensing_time BIGINT,
    product_id VARCHAR,
    cloud_cover VARCHAR,
    datatake_identifier VARCHAR,
    south_lat REAL COMMENT \'Southern latitude of the tile`s bounding box.\',
    generation_time BIGINT,
    west_lon REAL COMMENT \'Western longitude of the tile`s bounding box.\',
    base_url VARCHAR,
    granule_id VARCHAR,
    total_size BIGINT,
    east_lon REAL COMMENT \'Eastern longitude of the tile`s bounding box.\'
) WITH (format=\'ORC\');
'""")
flow.add_node(download_data_from_remote__wikitalk)


convert_csv_to_orc__sentinel = download_blob_to_file(
    'gcp-public-data-sentinel2',
    'index.csv.gz-backup',
    '/tmp/sentinel2',
    'sentinel-2-metadata-table')
flow.add_node(convert_csv_to_orc__sentinel)


replicated_data_sets__data_set_1 = ConstantTask('DownloadDataFromRemote')
flow.add_node(replicated_data_sets__data_set_1)
flow.add_edge(convert_csv_to_orc__sentinel, replicated_data_sets__data_set_1)
download_data_from_remote__sentinel = ConstantTask('DownloadDataFromRemote')
flow.add_node(download_data_from_remote__sentinel)


hive_schemas_created__storage_1__sentinel = ShellTask(
    command='gunzip {tmp_dir}/{file_name}'.format(
        tmp_dir='/tmp/sentinel2',
        file_name='sentinel-2-metadata-table'))
flow.add_node(hive_schemas_created__storage_1__sentinel)
flow.add_edge(convert_csv_to_orc__sentinel,
              hive_schemas_created__storage_1__sentinel)
replicated_schema__wikitalk = ShellTask(
    command='gunzip {tmp_dir}/{file_name}'.format(file_name='WikiTalk', tmp_dir='/tmp/wikitalk'))
flow.add_node(replicated_schema__wikitalk)
flow.add_edge(download_data_from_remote__sentinel, replicated_schema__wikitalk)


is_audited__basic_data_setup = ShellTask(command="""
tail -n +{n} {tmp_dir}/{file_name} > {tmp_dir}/{file_name}.no_header &&
rm {tmp_dir}/{file_name}
""".format(file_name='WikiTalk', n='2', tmp_dir='/tmp/wikitalk'))
flow.add_node(is_audited__basic_data_setup)
flow.add_edge(replicated_schema__wikitalk, is_audited__basic_data_setup)
replicated_schema__sentinel = ShellTask(command="""
tail -n +{n} {tmp_dir}/{file_name} > {tmp_dir}/{file_name}.no_header &&
rm {tmp_dir}/{file_name}
""".format(n='2', tmp_dir='/tmp/sentinel2', file_name='sentinel-2-metadata-table'))
flow.add_node(replicated_schema__sentinel)
flow.add_edge(
    hive_schemas_created__storage_1__sentinel,
    replicated_schema__sentinel)


decompress__wikitalk = ShellTask(
    command="""
csv-import {tmp_dir}/{table_name}.no_header {tmp_dir}/{table_name}.no_header
""".format(
        tmp_dir='/tmp/sentinel2',
        table_name='sentinel-2-metadata-table',
        schema=(
            'granule_id:STRING,product_id:STRING'
            ',datatake_identifier:STRING,mgrs_tile:STRING'
            ',sensing_time:BIGINT,total_size:BIGINT,cloud_cover:STRING'
            ',geometric_quality_flag:STRING,generation_time:BIGINT'
            ',north_lat:FLOAT,south_lat:FLOAT,west_lon:FLOAT')))
flow.add_node(decompress__wikitalk)
for dep in [replicated_data_sets__data_set_1, replicated_schema__sentinel]:
    flow.add_edge(dep, decompress__wikitalk)
remove_file_header__wikitalk = ShellTask(command="""
csv-import {tmp_dir}/{table_name}.no_header {tmp_dir}/{table_name}.no_header
""".format(tmp_dir='/tmp/wikitalk', schema='from_id:BIGINT,to_id:BIGINT', table_name='WikiTalk'))
flow.add_node(remove_file_header__wikitalk)
for dep in [download_data_from_remote__sentinel, is_audited__basic_data_setup]:
    flow.add_edge(dep, remove_file_header__wikitalk)


hive_schemas_created__sentinel = ConstantTask('ReplicatedData')
flow.add_node(hive_schemas_created__sentinel)
for dep in [
        convert_csv_to_orc__sentinel,
        replicated_data_sets__data_set_1,
        replicated_schema__sentinel,
        decompress__wikitalk]:
    flow.add_edge(dep, hive_schemas_created__sentinel)
decompress__sentinel = ConstantTask('ReplicatedData')
flow.add_node(decompress__sentinel)
for dep in [
        download_data_from_remote__sentinel,
        is_audited__basic_data_setup,
        remove_file_header__wikitalk]:
    flow.add_edge(dep, decompress__sentinel)


hive_schemas_created__storage_1__wikitalk = ConstantTask('ReplicatedSchema')
flow.add_node(hive_schemas_created__storage_1__wikitalk)
flow.add_edge(hive_schemas_created__wikitalk,
              hive_schemas_created__storage_1__wikitalk)
download_data_from_remote_gcs_location__sentinel_location = ConstantTask(
    'ReplicatedSchema')
flow.add_node(download_data_from_remote_gcs_location__sentinel_location)
flow.add_edge(download_data_from_remote__wikitalk,
              download_data_from_remote_gcs_location__sentinel_location)


convert_csv_to_orc__wikitalk = ConstantTask('ReplicatedDataSets')
flow.add_node(convert_csv_to_orc__wikitalk)
for dep in [decompress__sentinel, hive_schemas_created__storage_1__wikitalk]:
    flow.add_edge(dep, convert_csv_to_orc__wikitalk)
replicated_data_sets__snap = ConstantTask('ReplicatedDataSets')
flow.add_node(replicated_data_sets__snap)
for dep in [hive_schemas_created__sentinel,
            download_data_from_remote_gcs_location__sentinel_location]:
    flow.add_edge(dep, replicated_data_sets__snap)


remove_file_header__sentinel = ConstantTask('IsConsistent')
flow.add_node(remove_file_header__sentinel)
for dep in [convert_csv_to_orc__wikitalk, replicated_data_sets__snap]:
    flow.add_edge(dep, remove_file_header__sentinel)


replicated_data__sentinel = ConstantTask('IsAuditedTable')
flow.add_node(replicated_data__sentinel)
for dep in [convert_csv_to_orc__wikitalk, replicated_data_sets__snap]:
    flow.add_edge(dep, replicated_data__sentinel)
is_consistent__basic_data_setup = ConstantTask('IsAuditedTable')
flow.add_node(is_consistent__basic_data_setup)
for dep in [convert_csv_to_orc__wikitalk, replicated_data_sets__snap]:
    flow.add_edge(dep, is_consistent__basic_data_setup)


is_audited_table__wikitalk = ConstantTask('IsAudited')
flow.add_node(is_audited_table__wikitalk)
for dep in [replicated_data__sentinel, is_consistent__basic_data_setup]:
    flow.add_edge(dep, is_audited_table__wikitalk)
