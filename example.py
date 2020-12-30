hive_schemas_created__wikitalk = ShellTask(command="""
presto -e 'CREATE TABLE IF NOT EXISTS WikiTalk (
    from_id BIGINT,
    to_id BIGINT
) WITH (format=\'ORC\');
'""")
flow.add_node(hive_schemas_created__wikitalk)
hive_schemas_created__astroPh = ShellTask(command="""
presto -e 'CREATE TABLE IF NOT EXISTS ca-AstroPh (
    from_id BIGINT,
    to_id BIGINT
) WITH (format=\'ORC\');
'""")
flow.add_node(hive_schemas_created__astroPh)
hive_schemas_created__sentinel = ShellTask(command="""
presto -e 'CREATE TABLE IF NOT EXISTS sentinel-2-metadata-table (
    sensing_time BIGINT,
    base_url VARCHAR,
    product_id VARCHAR,
    west_lon REAL COMMENT \'Western longitude of the tile`s bounding box.\',
    total_size BIGINT,
    geometric_quality_flag VARCHAR,
    generation_time BIGINT,
    cloud_cover VARCHAR,
    granule_id VARCHAR,
    north_lat REAL COMMENT \'Northern latitude of the tile`s bounding box.\',
    datatake_identifier VARCHAR,
    south_lat REAL COMMENT \'Southern latitude of the tile`s bounding box.\',
    east_lon REAL COMMENT \'Eastern longitude of the tile`s bounding box.\',
    mgrs_tile VARCHAR
) WITH (format=\'ORC\');
'""")
flow.add_node(hive_schemas_created__sentinel)


replicated_schema__sentinel = ConstantTask('ReplicatedSchema')
flow.add_node(replicated_schema__sentinel)
flow.add_edge(hive_schemas_created__sentinel, replicated_schema__sentinel)
replicated_schema__astroPh = ConstantTask('ReplicatedSchema')
flow.add_node(replicated_schema__astroPh)
flow.add_edge(hive_schemas_created__astroPh, replicated_schema__astroPh)
replicated_schema__wikitalk = ConstantTask('ReplicatedSchema')
flow.add_node(replicated_schema__wikitalk)
flow.add_edge(hive_schemas_created__wikitalk, replicated_schema__wikitalk)


download_data_from_remote_gcs_location__sentinel_location = download_blob_to_file(
    'gcp-public-data-sentinel2',
    'index.csv.gz-backup',
    '/tmp/sentinel2',
    'sentinel-2-metadata-table')
flow.add_node(download_data_from_remote_gcs_location__sentinel_location)


download_data_from_remote__wikitalk = ConstantTask('DownloadDataFromRemote')
flow.add_node(download_data_from_remote__wikitalk)
download_data_from_remote__sentinel = ConstantTask('DownloadDataFromRemote')
flow.add_node(download_data_from_remote__sentinel)
flow.add_edge(
    download_data_from_remote_gcs_location__sentinel_location,
    download_data_from_remote__sentinel)
download_data_from_remote__astroPh = ConstantTask('DownloadDataFromRemote')
flow.add_node(download_data_from_remote__astroPh)


decompress__wikitalk = ShellTask(
    command='gunzip {tmp_dir}/{file_name}'.format(
        tmp_dir='/tmp/wikitalk',
        file_name='WikiTalk'))
flow.add_node(decompress__wikitalk)
flow.add_edge(download_data_from_remote__wikitalk, decompress__wikitalk)
decompress__astroPh = ShellTask(
    command='gunzip {tmp_dir}/{file_name}'.format(
        file_name='ca-AstroPh',
        tmp_dir='/tmp/wikitalk'))
flow.add_node(decompress__astroPh)
flow.add_edge(download_data_from_remote__astroPh, decompress__astroPh)
decompress__sentinel = ShellTask(
    command='gunzip {tmp_dir}/{file_name}'.format(
        tmp_dir='/tmp/sentinel2',
        file_name='sentinel-2-metadata-table'))
flow.add_node(decompress__sentinel)
flow.add_edge(
    download_data_from_remote_gcs_location__sentinel_location,
    decompress__sentinel)


remove_file_header__sentinel = ShellTask(command="""
tail -n +{n} {tmp_dir}/{file_name} > {tmp_dir}/{file_name}.no_header &&
rm {tmp_dir}/{file_name}
""".format(n='2', file_name='sentinel-2-metadata-table', tmp_dir='/tmp/sentinel2'))
flow.add_node(remove_file_header__sentinel)
flow.add_edge(decompress__sentinel, remove_file_header__sentinel)
remove_file_header__astroPh = ShellTask(command="""
tail -n +{n} {tmp_dir}/{file_name} > {tmp_dir}/{file_name}.no_header &&
rm {tmp_dir}/{file_name}
""".format(tmp_dir='/tmp/wikitalk', n='2', file_name='ca-AstroPh'))
flow.add_node(remove_file_header__astroPh)
flow.add_edge(decompress__astroPh, remove_file_header__astroPh)
remove_file_header__wikitalk = ShellTask(command="""
tail -n +{n} {tmp_dir}/{file_name} > {tmp_dir}/{file_name}.no_header &&
rm {tmp_dir}/{file_name}
""".format(n='2', tmp_dir='/tmp/wikitalk', file_name='WikiTalk'))
flow.add_node(remove_file_header__wikitalk)
flow.add_edge(decompress__wikitalk, remove_file_header__wikitalk)


data_from_remote_downloaded__astroPh = ConstantTask('DataFromRemoteDownloaded')
flow.add_node(data_from_remote_downloaded__astroPh)
for dep in [download_data_from_remote__astroPh, remove_file_header__astroPh]:
    flow.add_edge(dep, data_from_remote_downloaded__astroPh)
data_from_remote_downloaded__wikitalk = ConstantTask(
    'DataFromRemoteDownloaded')
flow.add_node(data_from_remote_downloaded__wikitalk)
for dep in [download_data_from_remote__wikitalk, remove_file_header__wikitalk]:
    flow.add_edge(dep, data_from_remote_downloaded__wikitalk)


convert_csv_to_orc__sentinel = ShellTask(
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
flow.add_node(convert_csv_to_orc__sentinel)
for dep in [download_data_from_remote__sentinel, remove_file_header__sentinel]:
    flow.add_edge(dep, convert_csv_to_orc__sentinel)
convert_csv_to_orc__wikitalk = ShellTask(command="""
csv-import {tmp_dir}/{table_name}.no_header {tmp_dir}/{table_name}.no_header
""".format(tmp_dir='/tmp/wikitalk', table_name='WikiTalk', schema='from_id:BIGINT,to_id:BIGINT'))
flow.add_node(convert_csv_to_orc__wikitalk)
for dep in [download_data_from_remote__wikitalk, remove_file_header__wikitalk]:
    flow.add_edge(dep, convert_csv_to_orc__wikitalk)
convert_csv_to_orc__astroPh = ShellTask(command="""
csv-import {tmp_dir}/{table_name}.no_header {tmp_dir}/{table_name}.no_header
""".format(table_name='ca-AstroPh', schema='from_id:BIGINT,to_id:BIGINT', tmp_dir='/tmp/wikitalk'))
flow.add_node(convert_csv_to_orc__astroPh)
for dep in [download_data_from_remote__astroPh, remove_file_header__astroPh]:
    flow.add_edge(dep, convert_csv_to_orc__astroPh)


upload_data_to_local__astroPh = ConstantTask('UploadDataToLocal')
flow.add_node(upload_data_to_local__astroPh)
flow.add_edge(convert_csv_to_orc__astroPh, upload_data_to_local__astroPh)
upload_data_to_local__wikitalk = ConstantTask('UploadDataToLocal')
flow.add_node(upload_data_to_local__wikitalk)
flow.add_edge(convert_csv_to_orc__wikitalk, upload_data_to_local__wikitalk)


replicated_data__sentinel = ConstantTask('ReplicatedData')
flow.add_node(replicated_data__sentinel)
for dep in [
        download_data_from_remote_gcs_location__sentinel_location,
        download_data_from_remote__sentinel,
        remove_file_header__sentinel,
        convert_csv_to_orc__sentinel]:
    flow.add_edge(dep, replicated_data__sentinel)


replicated_assets__wikitalk = ConstantTask('ReplicatedAssets')
flow.add_node(replicated_assets__wikitalk)
for dep in [
        replicated_schema__wikitalk,
        download_data_from_remote__wikitalk,
        data_from_remote_downloaded__wikitalk,
        upload_data_to_local__wikitalk]:
    flow.add_edge(dep, replicated_assets__wikitalk)
replicated_assets__astroPh = ConstantTask('ReplicatedAssets')
flow.add_node(replicated_assets__astroPh)
for dep in [
        replicated_schema__astroPh,
        download_data_from_remote__astroPh,
        data_from_remote_downloaded__astroPh,
        upload_data_to_local__astroPh]:
    flow.add_edge(dep, replicated_assets__astroPh)


replicated_data_sets__snap = ConstantTask('ReplicatedDataSets')
flow.add_node(replicated_data_sets__snap)
for dep in [replicated_assets__wikitalk, replicated_assets__astroPh]:
    flow.add_edge(dep, replicated_data_sets__snap)
replicated_data_sets__sentinel = ConstantTask('ReplicatedDataSets')
flow.add_node(replicated_data_sets__sentinel)
for dep in [replicated_schema__sentinel, replicated_data__sentinel]:
    flow.add_edge(dep, replicated_data_sets__sentinel)


is_consistent__basic_data_setup = ConstantTask('IsConsistent')
flow.add_node(is_consistent__basic_data_setup)
for dep in [replicated_data_sets__snap, replicated_data_sets__sentinel]:
    flow.add_edge(dep, is_consistent__basic_data_setup)


is_audited_table__astroPh = ConstantTask('IsAuditedTable')
flow.add_node(is_audited_table__astroPh)
for dep in [replicated_data_sets__snap, replicated_data_sets__sentinel]:
    flow.add_edge(dep, is_audited_table__astroPh)
is_audited_table__wikitalk = ConstantTask('IsAuditedTable')
flow.add_node(is_audited_table__wikitalk)
for dep in [replicated_data_sets__snap, replicated_data_sets__sentinel]:
    flow.add_edge(dep, is_audited_table__wikitalk)
is_audited_table__sentinel = ConstantTask('IsAuditedTable')
flow.add_node(is_audited_table__sentinel)
for dep in [replicated_data_sets__snap, replicated_data_sets__sentinel]:
    flow.add_edge(dep, is_audited_table__sentinel)


is_audited__basic_data_setup = ConstantTask('IsAudited')
flow.add_node(is_audited__basic_data_setup)
for dep in [
        is_audited_table__astroPh,
        is_audited_table__wikitalk,
        is_audited_table__sentinel]:
    flow.add_edge(dep, is_audited__basic_data_setup)
