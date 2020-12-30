download_location = download_blob_to_file('gcp-public-data-sentinel2', 'index.csv.gz-backup', '/tmp/sentinel2', 'sentinel-2-metadata-table')
flow.add_node(download_location)
hive_created = ShellTask(command="""
CREATE TABLE IF NOT EXISTS {table_name} (
    {schema}
) WITH (format='{data_format}');
""".format(data_format='ORC', schema="""
generation_time BIGINT,
west_lon REAL COMMENT 'Western longitude of the tile`s bounding box.',
geometric_quality_flag VARCHAR,
base_url VARCHAR,
east_lon REAL COMMENT 'Eastern longitude of the tile`s bounding box.',
granule_id VARCHAR,
product_id VARCHAR,
datatake_identifier VARCHAR,
sensing_time BIGINT,
total_size BIGINT,
cloud_cover VARCHAR,
mgrs_tile VARCHAR,
north_lat REAL COMMENT 'Northern latitude of the tile`s bounding box.',
south_lat REAL COMMENT 'Southern latitude of the tile`s bounding box.'
""", table_name='sentinel-2-metadata-table'))
flow.add_node(hive_created)
replicated_schema = ConstantTask('ReplicatedSchema')
flow.add_node(replicated_schema)
flow.add_edge(hive_created, replicated_schema)
download_remote = ConstantTask('DownloadDataFromRemote')
flow.add_node(download_remote)
flow.add_edge(download_location, download_remote)
decompress = ShellTask(command='gunzip {tmp_dir}/{file_name}'.format(file_name='sentinel-2-metadata-table', tmp_dir='/tmp/sentinel2'))
flow.add_node(decompress)
flow.add_edge(download_location, decompress)

remove_header = ShellTask(command="""
tail -n +2 {tmp_dir}/{file_name} > {tmp_dir}/{file_name}.no_header &&
rm {tmp_dir}/{file_name}
""".format(tmp_dir='/tmp/sentinel2', file_name='sentinel-2-metadata-table'))
flow.add_node(remove_header)
flow.add_edge(decompress, remove_header)

convert_orc = ShellTask(command="""
csv-import {tmp_dir}/{table_name}.no_header {tmp_dir}/{table_name}.no_header
""".format(table_name='sentinel-2-metadata-table', schema='granule_id:STRING,product_id:STRING,datatake_identifier:STRING,mgrs_tile:STRING,sensing_time:BIGINT,total_size:BIGINT,cloud_cover:STRING,geometric_quality_flag:STRING,generation_time:BIGINT,north_lat:FLOAT,south_lat:FLOAT,west_lon:FLOAT,east_lon:FLOAT,base_url:STRING', tmp_dir='/tmp/sentinel2'))
flow.add_node(convert_orc)
for dep in [download_remote, remove_header]:
    flow.add_edge(dep, convert_orc)

replicated_data = ConstantTask('ReplicatedData')
flow.add_node(replicated_data)
for dep in [download_location, download_remote, remove_header, convert_orc]:
    flow.add_edge(dep, replicated_data)


is_consistent = ConstantTask('IsConsistent')
flow.add_node(is_consistent)
for dep in [replicated_schema, replicated_data]:
    flow.add_edge(dep, is_consistent)

is_audited = ConstantTask('IsAudited')
flow.add_node(is_audited)
for dep in [replicated_schema, replicated_data]:
    flow.add_edge(dep, is_audited)
