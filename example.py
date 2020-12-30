hive_schemas_created____storage_1__sentinel = ShellTask(command="""
presto -e 'CREATE TABLE IF NOT EXISTS sentinel-2-metadata-table (
    geometric_quality_flag VARCHAR,
    base_url VARCHAR,
    sensing_time BIGINT,
    granule_id VARCHAR,
    west_lon REAL COMMENT \'Western longitude of the tile`s bounding box.\',
    generation_time BIGINT,
    mgrs_tile VARCHAR,
    product_id VARCHAR,
    north_lat REAL COMMENT \'Northern latitude of the tile`s bounding box.\',
    south_lat REAL COMMENT \'Southern latitude of the tile`s bounding box.\',
    total_size BIGINT,
    datatake_identifier VARCHAR,
    cloud_cover VARCHAR,
    east_lon REAL COMMENT \'Eastern longitude of the tile`s bounding box.\'
) WITH (format=\'ORC\');
'""")
flow.add_node(hive_schemas_created____storage_1__sentinel)
hive_schemas_created____storage_1__wikitalk = ShellTask(command="""
presto -e 'CREATE TABLE IF NOT EXISTS WikiTalk (
    to_id BIGINT,
    from_id BIGINT
) WITH (format=\'ORC\');
'""")
flow.add_node(hive_schemas_created____storage_1__wikitalk)


download_data_from_remote_gcs_location__sentinel_location = download_blob_to_file(
    'gcp-public-data-sentinel2',
    'index.csv.gz-backup',
    '/tmp/sentinel2',
    'sentinel-2-metadata-table')
flow.add_node(download_data_from_remote_gcs_location__sentinel_location)


download_data_from_remote____sentinel = ConstantTask('DownloadDataFromRemote')
flow.add_node(download_data_from_remote____sentinel)
flow.add_edge(
    download_data_from_remote_gcs_location__sentinel_location,
    download_data_from_remote____sentinel)
download_data_from_remote____wikitalk = ConstantTask('DownloadDataFromRemote')
flow.add_node(download_data_from_remote____wikitalk)


decompress____wikitalk = ShellTask(
    command='gunzip {tmp_dir}/{file_name}'.format(
        file_name='WikiTalk',
        tmp_dir='/tmp/wikitalk'))
flow.add_node(decompress____wikitalk)
flow.add_edge(download_data_from_remote____wikitalk, decompress____wikitalk)
decompress____sentinel = ShellTask(
    command='gunzip {tmp_dir}/{file_name}'.format(
        file_name='sentinel-2-metadata-table',
        tmp_dir='/tmp/sentinel2'))
flow.add_node(decompress____sentinel)
flow.add_edge(
    download_data_from_remote_gcs_location__sentinel_location,
    decompress____sentinel)


remove_file_header____wikitalk = ShellTask(command="""
tail -n +{n} {tmp_dir}/{file_name} > {tmp_dir}/{file_name}.no_header &&
rm {tmp_dir}/{file_name}
""".format(file_name='WikiTalk', n='2', tmp_dir='/tmp/wikitalk'))
flow.add_node(remove_file_header____wikitalk)
flow.add_edge(decompress____wikitalk, remove_file_header____wikitalk)
remove_file_header____sentinel = ShellTask(command="""
tail -n +{n} {tmp_dir}/{file_name} > {tmp_dir}/{file_name}.no_header &&
rm {tmp_dir}/{file_name}
""".format(tmp_dir='/tmp/sentinel2', n='2', file_name='sentinel-2-metadata-table'))
flow.add_node(remove_file_header____sentinel)
flow.add_edge(decompress____sentinel, remove_file_header____sentinel)


replicated_schema__sentinel = ConstantTask('ReplicatedSchema')
flow.add_node(replicated_schema__sentinel)
flow.add_edge(
    hive_schemas_created____storage_1__sentinel,
    replicated_schema__sentinel)
replicated_schema__wikitalk = ConstantTask('ReplicatedSchema')
flow.add_node(replicated_schema__wikitalk)
flow.add_edge(
    hive_schemas_created____storage_1__wikitalk,
    replicated_schema__wikitalk)


convert_csv_to_orc____sentinel = ShellTask(
    command="""
csv-import {tmp_dir}/{table_name}.no_header {tmp_dir}/{table_name}.no_header
""".format(
        table_name='sentinel-2-metadata-table',
        tmp_dir='/tmp/sentinel2',
        schema=(
            'granule_id:STRING,product_id:STRING'
            ',datatake_identifier:STRING,mgrs_tile:STRING'
            ',sensing_time:BIGINT,total_size:BIGINT,cloud_cover:STRING'
            ',geometric_quality_flag:STRING,generation_time:BIGINT'
            ',north_lat:FLOAT,south_lat:FLOAT,west_lon:FLOAT')))
flow.add_node(convert_csv_to_orc____sentinel)
for dep in [
        download_data_from_remote____sentinel,
        remove_file_header____sentinel]:
    flow.add_edge(dep, convert_csv_to_orc____sentinel)
convert_csv_to_orc____wikitalk = ShellTask(command="""
csv-import {tmp_dir}/{table_name}.no_header {tmp_dir}/{table_name}.no_header
""".format(tmp_dir='/tmp/wikitalk', schema='from_id:BIGINT,to_id:BIGINT', table_name='WikiTalk'))
flow.add_node(convert_csv_to_orc____wikitalk)
for dep in [
        download_data_from_remote____wikitalk,
        remove_file_header____wikitalk]:
    flow.add_edge(dep, convert_csv_to_orc____wikitalk)


replicated_data____wikitalk = ConstantTask('ReplicatedData')
flow.add_node(replicated_data____wikitalk)
for dep in [
        download_data_from_remote____wikitalk,
        remove_file_header____wikitalk,
        convert_csv_to_orc____wikitalk]:
    flow.add_edge(dep, replicated_data____wikitalk)
replicated_data____sentinel = ConstantTask('ReplicatedData')
flow.add_node(replicated_data____sentinel)
for dep in [
        download_data_from_remote_gcs_location__sentinel_location,
        download_data_from_remote____sentinel,
        remove_file_header____sentinel,
        convert_csv_to_orc____sentinel]:
    flow.add_edge(dep, replicated_data____sentinel)


replicated_data_sets__snap = ConstantTask('ReplicatedDataSets')
flow.add_node(replicated_data_sets__snap)
for dep in [replicated_schema__wikitalk, replicated_data____wikitalk]:
    flow.add_edge(dep, replicated_data_sets__snap)
replicated_data_sets____data_set_1 = ConstantTask('ReplicatedDataSets')
flow.add_node(replicated_data_sets____data_set_1)
for dep in [replicated_schema__sentinel, replicated_data____sentinel]:
    flow.add_edge(dep, replicated_data_sets____data_set_1)


is_consistent__basic_data_setup = ConstantTask('IsConsistent')
flow.add_node(is_consistent__basic_data_setup)
for dep in [replicated_data_sets__snap, replicated_data_sets____data_set_1]:
    flow.add_edge(dep, is_consistent__basic_data_setup)


is_audited_table__sentinel = ConstantTask('IsAuditedTable')
flow.add_node(is_audited_table__sentinel)
for dep in [replicated_data_sets__snap, replicated_data_sets____data_set_1]:
    flow.add_edge(dep, is_audited_table__sentinel)
is_audited_table__wikitalk = ConstantTask('IsAuditedTable')
flow.add_node(is_audited_table__wikitalk)
for dep in [replicated_data_sets__snap, replicated_data_sets____data_set_1]:
    flow.add_edge(dep, is_audited_table__wikitalk)


is_audited__basic_data_setup = ConstantTask('IsAudited')
flow.add_node(is_audited__basic_data_setup)
for dep in [is_audited_table__sentinel, is_audited_table__wikitalk]:
    flow.add_edge(dep, is_audited__basic_data_setup)
